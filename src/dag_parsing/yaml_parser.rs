use super::dag_config::{
    DagConfig, DefinitionConfig, DefinitionConfigId, NodeConfig, NodeConfigId, RunnerConfig,
    RunnerConfigId,
};
use super::dag_config_parser::{DagConfigParser, SupportedFormat};
use crate::dag_parsing::DagParsingError;
use log::{debug, info};
use serde::Deserialize;
use serde_yaml;
use serde_yaml::Value;
use std::collections::{BTreeMap, HashMap};

/*
Interesting article about serde and validation:
https://blog.logrocket.com/json-input-validation-in-rust-web-services/

https://play.org/articles/effective-serde-by-writing-less-rust-code


I'm not sure I'm doing that well :s
*/

/// Serde-derive: runner when in a node
#[derive(Debug, PartialEq, Deserialize, Clone)]
struct ParsedYamlNodeRunner {
    id_runner: RunnerConfigId,
}

/// Serde-derive: definition when in a node
#[derive(Debug, PartialEq, Deserialize, Clone)]
struct ParsedYamlNodeDefinition {
    id_task: DefinitionConfigId,

    #[serde(flatten)]
    params: HashMap<String, Value>,
}

/// Serde-derive: node
#[derive(Debug, PartialEq, Deserialize)]
struct ParsedYamlNode {
    ref_definition: Option<String>,
    ref_runner: Option<String>,
    definition: Option<ParsedYamlNodeDefinition>,
    runner: Option<ParsedYamlNodeRunner>,
    child: Option<String>,
    children: Option<Vec<String>>,
}

/// Serde-derive: definition
#[derive(Debug, PartialEq, Deserialize)]
struct ParsedYamlDefinition {
    id_task: String,

    #[serde(flatten)]
    params: HashMap<String, Value>,
}

/// Serde-derive: runner
#[derive(Debug, PartialEq, Deserialize)]
struct ParsedYamlRunner {
    id_runner: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
enum ParsedYamlDagEdges {
    Child(NodeConfigId),
    Children(Vec<NodeConfigId>),
}

/// Serde-derive: whole parsed config
#[derive(Debug, PartialEq, Deserialize)]
struct ParsedYamlConfig {
    nodes: BTreeMap<String, ParsedYamlNode>,
    definitions: Option<BTreeMap<String, ParsedYamlDefinition>>,
    runners: Option<BTreeMap<String, ParsedYamlRunner>>,
    dag: Option<BTreeMap<String, ParsedYamlDagEdges>>,
}

pub struct YamlDagConfigParser {}

// todo: move to a module?
/// Convert a serde_yaml::Value to a String
fn convert_yaml_value_to_string(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(boolean) => {
            if *boolean {
                return "true".to_string();
            } else {
                return "false".to_string();
            }
        }
        Value::Number(num) => num.to_string(),
        Value::String(string) => string.to_string(),
        Value::Sequence(seq) => {
            let mut buffer = String::from("[");
            for el in seq.iter() {
                buffer.push_str(&convert_yaml_value_to_string(el));
            }
            buffer.push(']');
            return buffer;
        }
        Value::Mapping(map) => {
            let mut buffer = String::from("{");
            for (key, val) in map.iter() {
                let format_pair = format!(
                    "{}: {}",
                    convert_yaml_value_to_string(key),
                    convert_yaml_value_to_string(val)
                );
                buffer.push_str(&format_pair);
            }
            buffer.push('}');
            return buffer;
        }
    }
}

/// Build a DagConfig from a parsed yaml
fn build_dag_config(config: ParsedYamlConfig) -> Result<DagConfig, DagParsingError> {
    debug!("{:#?}", config);
    let mut map_nodes: HashMap<NodeConfigId, NodeConfig> = HashMap::new();
    let mut map_definitions: HashMap<DefinitionConfigId, DefinitionConfig> = HashMap::new();
    let mut map_runners: HashMap<RunnerConfigId, RunnerConfig> = HashMap::new();
    let mut dag_edges: HashMap<NodeConfigId, Vec<NodeConfigId>> = HashMap::new();

    // todo: try reducing number of .to_string()
    // RUNNERS
    info!("Looking for runner entries");
    if config.runners.is_some() {
        for (runner_name, parsed_runner) in config.runners.unwrap().iter() {
            let runner_cfg = RunnerConfig {
                id_runner: runner_name.to_string(),
                runner_type: parsed_runner.id_runner.clone(),
            };
            map_runners.insert(runner_name.to_string(), runner_cfg);
        }
    }

    // DEFINITIONS
    info!("Looking for definition entries");
    if config.definitions.is_some() {
        for (def_name, parsed_def) in config.definitions.unwrap().iter() {
            let mut hashmap_params = HashMap::<String, String>::new();
            if !parsed_def.params.is_empty() {
                for (key, value) in parsed_def.params.iter() {
                    let value_string = convert_yaml_value_to_string(value);
                    hashmap_params.insert(key.clone(), value_string);
                }
            }
            let def_cfg = DefinitionConfig {
                id_definition: def_name.to_string(),
                definition_type: parsed_def.id_task.clone(),
                params: hashmap_params,
            };
            map_definitions.insert(def_name.to_string(), def_cfg);
        }
    }

    // NODES
    info!("Processing node entries");
    if config.nodes.len() == 0 {
        // todo: replace errors in this function by more specific ones
        return Err(DagParsingError {
            reason: String::from("Parsed config has no nodes"),
        });
    }
    for (node_name, parsed_node) in config.nodes.iter() {
        // Node--Definition
        // parsed node cannot contain both definition and ref_definition
        if parsed_node.ref_definition.is_some() && parsed_node.definition.is_some() {
            return Err(DagParsingError {
                reason: format!(
                    "Node '{}' cannot have both 'ref_definition' and 'definition' sections",
                    node_name
                ),
            });
        }
        let used_ref_definition = match &parsed_node.definition {
            Some(node_def) => {
                // add node def to existing map
                let node_def_id = format!("{}_{}", node_name.to_string(), node_def.id_task.clone());
                let mut hashmap_params = HashMap::<String, String>::new();
                if !node_def.params.is_empty() {
                    for (key, value) in node_def.params.iter() {
                        let value_string = convert_yaml_value_to_string(value);
                        hashmap_params.insert(key.clone(), value_string);
                    }
                }
                let cfg_def = DefinitionConfig {
                    id_definition: node_def_id.clone(),
                    definition_type: node_def.id_task.clone(),
                    params: hashmap_params,
                };
                map_definitions.insert(node_def_id.clone(), cfg_def);
                node_def_id.to_string()
            }
            None => {
                // check that referenced definition is in map
                let ref_def = parsed_node.ref_definition.as_ref().unwrap().to_string();
                if !map_definitions.contains_key(&ref_def) {
                    return Err(DagParsingError {
                        reason: format!(
                            "Ref definition '{}' in '{}' not known",
                            ref_def, node_name
                        ),
                    });
                } else {
                    ref_def
                }
            }
        };

        // Node--Runner
        // parsed node cannot contain both runner and ref_runner
        if parsed_node.ref_runner.is_some() && parsed_node.runner.is_some() {
            return Err(DagParsingError {
                reason: format!(
                    "Node '{}' cannot have both 'ref_runner' and 'runner' sections",
                    node_name
                ),
            });
        }
        // todo: it's the same thing as definition, should be refactored in one function?
        let used_ref_runner = match &parsed_node.runner {
            Some(node_def) => {
                let node_runner_id =
                    format!("{}_{}", node_name.to_string(), node_def.id_runner.clone());
                let cfg_runner = RunnerConfig {
                    id_runner: node_runner_id.clone(),
                    runner_type: node_def.id_runner.clone(),
                };
                map_runners.insert(node_runner_id.clone(), cfg_runner);
                node_runner_id.to_string()
            }
            None => {
                let ref_runner = parsed_node.ref_runner.as_ref().unwrap().to_string();
                if !map_runners.contains_key(&ref_runner) {
                    return Err(DagParsingError {
                        reason: format!("Ref runner '{}' in '{}' not known", ref_runner, node_name),
                    });
                } else {
                    ref_runner
                }
            }
        };

        // Node--Children
        if parsed_node.child.is_some() {
            let child = vec![parsed_node.child.clone().unwrap()];
            dag_edges.insert(node_name.to_string(), child);
        }
        if parsed_node.children.is_some() {
            let children = parsed_node.children.clone().unwrap();
            dag_edges.insert(node_name.to_string(), children);
        }

        //
        let node_cfg = NodeConfig {
            id_node: node_name.to_string(),
            ref_definition: used_ref_definition,
            ref_runner: used_ref_runner,
        };
        map_nodes.insert(node_name.to_string(), node_cfg);
    }

    // DAG EDGES
    info!("Looking for DAG edges");
    if config.dag.is_some() {
        for (parent_node, node_children) in config.dag.unwrap().iter() {
            if !map_nodes.contains_key(parent_node) {
                return Err(DagParsingError {
                    reason: format!("Adding children to not known node '{:?}'", parent_node),
                });
            }
            match node_children {
                ParsedYamlDagEdges::Child(child) => {
                    if !map_nodes.contains_key(child) {
                        return Err(DagParsingError {
                            reason: format!(
                                "Adding not known node '{:?}' to '{:?}'",
                                child, parent_node
                            ),
                        });
                    }
                    dag_edges.insert(parent_node.to_string(), vec![child.to_string()]);
                }
                ParsedYamlDagEdges::Children(children) => {
                    for child in children.iter() {
                        if !map_nodes.contains_key(child) {
                            return Err(DagParsingError {
                                reason: format!(
                                    "Adding not known node '{:?}' to '{:?}'",
                                    child, parent_node
                                ),
                            });
                        }
                    }
                    dag_edges.insert(parent_node.to_string(), children.to_vec());
                }
            }
        }
    }

    Ok(DagConfig {
        nodes: map_nodes,
        definitions: map_definitions,
        runners: map_runners,
        dag_edges,
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
            return false;
        }
        let deserialized_map: ParsedYamlConfig = serde_yaml::from_str(&content).unwrap();

        true
    }

    fn parse_file(&self, content: String) -> Result<DagConfig, DagParsingError> {
        info!("Creating DagConfig from YAML");
        let parsed_content = serde_yaml::from_str(&content).unwrap();
        build_dag_config(parsed_content)
    }
}

#[cfg(test)]
#[path = "./yaml_parser_test.rs"]
mod yaml_parser_test;
