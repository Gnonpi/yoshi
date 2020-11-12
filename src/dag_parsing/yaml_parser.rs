use super::dag_config_parser::{SupportedFormat, DagConfigParser};
use super::dag_config::{
    DagConfig, 
    NodeConfigId,
    DefinitionConfigId,
    RunnerDefinitionId,
    NodeConfig,
    DefinitionConfig,
    RunnerDefinition,
};
use crate::dag_parsing::DagParsingError;
use std::collections::{BTreeMap, HashMap};
use serde::Deserialize;
use serde_yaml;
use log::info;

/*
Interesting article about serde and validation:
https://blog.logrocket.com/json-input-validation-in-rust-web-services/
*/

#[derive(Debug, PartialEq, Deserialize)]
struct ParsedYamlNode {
  ref_definition: Option<String>,
  ref_runner: Option<String>,
  runner: Option<BTreeMap<String, String>>
}

#[derive(Debug, PartialEq, Deserialize)]
struct ParsedYamlDefinition {
  id_task: String
}

#[derive(Debug, PartialEq, Deserialize)]
struct ParsedYamlRunner {
  id_runner: String
}

#[derive(Debug, PartialEq, Deserialize)]
struct ParsedYamlConfig {
  nodes: BTreeMap<String, ParsedYamlNode>,
  definitions: BTreeMap<String, ParsedYamlDefinition>,
  runners: BTreeMap<String, ParsedYamlRunner>,
  dag: BTreeMap<String, Vec<String>>
}

pub struct YamlDagConfigParser {}

impl YamlDagConfigParser {
    fn build_dag_config(&self) -> Result<DagConfig, DagParsingError> {
        let mut map_nodes: HashMap<NodeConfigId, NodeConfig> = HashMap::new();
        let mut map_definitions: HashMap<DefinitionConfigId, DefinitionConfig> = HashMap::new();
        let mut runners: HashMap<RunnerDefinitionId, RunnerDefinition> = HashMap::new();
        let mut dag_edges: HashMap<NodeConfigId, Vec<NodeConfigId>> = HashMap::new();

        if self.nodes.len() == 0 {
            return Err(DagParsingError)
        }
        for (node_name, node_cfg) in self.nodes.iter() {
            
        }
    }
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
        Err(DagParsingError {reason: String::from("to impl")})
    }
}

#[cfg(test)]
#[path = "./yaml_parser_test.rs"]
mod yaml_parser_test;

