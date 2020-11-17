use super::dag_config_parser::{SupportedFormat, DagConfigParser};
use super::dag_config::{
    DagConfig, 
    NodeConfigId,
    DefinitionConfigId,
    RunnerConfigId,
    NodeConfig,
    DefinitionConfig,
    RunnerConfig,
};
use crate::dag_parsing::DagParsingError;
use std::collections::{BTreeMap, HashMap};
use serde::Deserialize;
use serde_yaml;
use log::info;

/*
Interesting article about serde and validation:
https://blog.logrocket.com/json-input-validation-in-rust-web-services/

https://play.org/articles/effective-serde-by-writing-less-rust-code


I'm not sure I'm doing that well :s
*/

/// Serde-derive: runner when in a node
#[derive(Debug, PartialEq, Deserialize, Clone)]
struct ParsedYamlNodeRunner {
    id_runner: RunnerConfigId
}

/// Serde-derive: definition when in a node
#[derive(Debug, PartialEq, Deserialize, Clone)]
struct ParsedYamlNodeDefinition {
    id_task: DefinitionConfigId,
    
    #[serde(flatten)]
    params: HashMap<String, String>
}

/// Serde-derive: node
#[derive(Debug, PartialEq, Deserialize)]
struct ParsedYamlNode {
  ref_definition: Option<String>,
  ref_runner: Option<String>,
  definition: Option<BTreeMap<String, ParsedYamlNodeDefinition>>,
  runner: Option<BTreeMap<String, ParsedYamlNodeRunner>>,
  child: Option<String>,
  children: Option<Vec<String>>
}

/// Serde-derive: definition
#[derive(Debug, PartialEq, Deserialize)]
struct ParsedYamlDefinition {
  id_task: String,
  
  #[serde(flatten)]
  params: Option<BTreeMap<String, String>>
}

/// Serde-derive: runner
#[derive(Debug, PartialEq, Deserialize)]
struct ParsedYamlRunner {
  id_runner: String
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
enum ParsedYamlDagEdges {
    Child(NodeConfigId),
    Children(Vec<NodeConfigId>)
}

/// Serde-derive: whole parsed config
#[derive(Debug, PartialEq, Deserialize)]
struct ParsedYamlConfig {
  nodes: BTreeMap<String, ParsedYamlNode>,
  definitions: BTreeMap<String, ParsedYamlDefinition>,
  runners: BTreeMap<String, ParsedYamlRunner>,
  dag: BTreeMap<String, ParsedYamlDagEdges>
}

pub struct YamlDagConfigParser {}

/// To avoid working in nightly,
/// implement a simple BTreeMap.first_entry_value
fn first_entry_value<K: Clone, V: Clone>(btree: &BTreeMap<K, V>) -> Option<(K, V)> {
    if btree.len() == 0 {
        return None
    }
    let (key, value) = btree.iter().next().unwrap();
    Some((key.clone(), value.clone()))
}


/// Build a DagConfig from a parsed yaml
fn build_dag_config(config: ParsedYamlConfig) -> Result<DagConfig, DagParsingError> {
    let mut map_nodes: HashMap<NodeConfigId, NodeConfig> = HashMap::new();
    let mut map_definitions: HashMap<DefinitionConfigId, DefinitionConfig> = HashMap::new();
    let mut map_runners: HashMap<RunnerConfigId, RunnerConfig> = HashMap::new();
    let mut dag_edges: HashMap<NodeConfigId, Vec<NodeConfigId>> = HashMap::new();

    // todo: try reducing number of .to_string()
    // RUNNERS
    for (runner_name, parsed_runner) in config.runners.iter() {
        let runner_cfg = RunnerConfig {
            id_runner: runner_name.to_string(),
            runner_type: parsed_runner.id_runner.clone()
        };
        map_runners.insert(runner_name.to_string(), runner_cfg);
    }

    // DEFINITIONS
    for (def_name, parsed_def) in config.definitions.iter() {
        let mut hashmap_params = HashMap::<String, String>::new();
        if parsed_def.params.is_some() {
            for (key, value) in parsed_def.params.clone().unwrap().iter() {
                hashmap_params.insert(key.clone(), value.clone());
            }
        }
        let def_cfg = DefinitionConfig {
            id_definition: def_name.to_string(),
            definition_type: parsed_def.id_task.clone(),
            params: hashmap_params
        };
        map_definitions.insert(def_name.to_string(), def_cfg);
    }

    // NODES
    if config.nodes.len() == 0 {
        // todo: replace errors in this function by more specific ones
        return Err(DagParsingError { reason: String::from("Parsed config has no nodes") })
    }
    for (node_name, parsed_node) in config.nodes.iter() {
        // Node--Definition
        // parsed node cannot contain both definition and ref_definition
        if parsed_node.ref_definition.is_some() && parsed_node.definition.is_some() {
            return Err(DagParsingError { reason: format!("Node '{}' cannot have both 'ref_definition' and 'definition' sections", node_name) })
        }
        let used_ref_definition = match &parsed_node.definition {
            Some(node_def) => {
                // add node def to existing map
                let (def_name, def_value) = first_entry_value(&node_def).unwrap();
                let cfg_def = DefinitionConfig {
                    id_definition: def_name.to_string(),
                    definition_type: def_value.id_task,
                    params: def_value.params
                };
                map_definitions.insert(def_name.to_string(), cfg_def);
                def_name.to_string()
            },
            None => {
                // check that referenced definition is in map
                let ref_def = parsed_node.ref_definition.as_ref().unwrap().to_string();
                if !map_definitions.contains_key(&ref_def) {
                    return Err(DagParsingError { 
                        reason: format!("Ref definition '{}' in '{}' not known", ref_def, node_name) 
                    })    
                } else {
                    ref_def
                }
            }
        };

        // Node--Runner
        // parsed node cannot contain both runner and ref_runner
        if parsed_node.ref_runner.is_some() && parsed_node.runner.is_some()  {
            return Err(DagParsingError { reason: format!("Node '{}' cannot have both 'ref_runner' and 'runner' sections", node_name) })
        }
        // todo: it's the same thing as definition, should be refactored in one function?
        let used_ref_runner = match &parsed_node.runner {
            Some(node_def) => {
                let (runner_name, runner_value) = first_entry_value(&node_def).unwrap();
                let cfg_runner = RunnerConfig {
                    id_runner: runner_name.to_string(),
                    runner_type: runner_value.id_runner
                };
                map_runners.insert(runner_name.to_string(), cfg_runner);
                runner_name.to_string()
            },
            None => {
                let ref_runner = parsed_node.ref_runner.as_ref().unwrap().to_string();
                if !map_runners.contains_key(&ref_runner) {
                    return Err(DagParsingError {
                        reason: format!("Ref runner '{}' in '{}' not known", ref_runner, node_name)
                    })
                } else {
                    ref_runner
                }
            }         
        };

        // Node--Children
        // if parsed_node.child.is_some() 

        // 
        let node_cfg = NodeConfig {
            id_node: node_name.to_string(),
            ref_definition: used_ref_definition,
            ref_runner: used_ref_runner
        };
        map_nodes.insert(node_name.to_string(), node_cfg);
    }

    // DAG EDGES
    for (parent_node, node_children) in config.dag.iter() {
        if !map_nodes.contains_key(parent_node) {
            return Err(DagParsingError { reason: format!("Adding children to not known node '{:?}'", parent_node)})
        }
        match node_children {
            ParsedYamlDagEdges::Child(child) => {
                if !map_nodes.contains_key(child) {
                    return Err(DagParsingError { reason: format!("Adding not known node '{:?}' to '{:?}'", child, parent_node)})
                }   
                dag_edges.insert(parent_node.to_string(), vec![child.to_string()]);             
            },
            ParsedYamlDagEdges::Children(children) => {
                for child in children.iter() {
                    if !map_nodes.contains_key(child) {
                        return Err(DagParsingError { reason: format!("Adding not known node '{:?}' to '{:?}'", child, parent_node)})    
                    }
                }
                dag_edges.insert(parent_node.to_string(), children.to_vec());
            }
        }
    }

    Ok(DagConfig {
      nodes: map_nodes,
      definitions: map_definitions,
      runners: map_runners,
      dag_edges
    })
}

impl DagConfigParser for YamlDagConfigParser {
    fn get_format(&self) -> SupportedFormat {
        SupportedFormat::Yaml
    }
    
    fn validate(&self, content: &String) -> bool {
      // Trying to load a Yaml
      let read_yaml: serde_yaml::Result<ParsedYamlConfig> = serde_yaml::from_str(&content);
      if read_yaml.is_err() {
        info!("Could not parse Yaml file");
        return false
      }
      let deserialized_map: ParsedYamlConfig = serde_yaml::from_str(&content).unwrap();
      
      true
    }
    
    fn parse_file(&self, content: String) -> Result<DagConfig, DagParsingError> {
        let parsed_content = serde_yaml::from_str(&content).unwrap();
        build_dag_config(parsed_content)
    }
}

#[cfg(test)]
#[path = "./yaml_parser_test.rs"]
mod yaml_parser_test;

