use super::dag_config_parser::{SupportedFormat, DagConfigParser};
use super::dag_config::DagConfig;
use crate::dag_parsing::DagParsingError;
use yaml_rust::YamlLoader;
use yaml_rust::Yaml::Hash;
use log::info;

/*
Interesting article about serde and validation:
https://blog.logrocket.com/json-input-validation-in-rust-web-services/
*/

pub struct YamlDagConfigParser {}


impl DagConfigParser for YamlDagConfigParser {
    fn get_format(&self) -> SupportedFormat {
        SupportedFormat::Yaml
    }
    
    fn validate(&self, content: &String) -> bool {
      // Trying to load a Yaml
        let loading_yaml = YamlLoader::load_from_str(content.as_str());
        if loading_yaml.is_err() {
            info!("Content is not a valid YAML: {:?}", loading_yaml.unwrap_err());
            return false
        }
        

    }
    
    fn parse_file(&self, content: String) -> Result<DagConfig, DagParsingError> {
        Err(DagParsingError {reason: String::from("to impl")})
    }
}

#[cfg(test)]
#[path = "./yaml_parser_test.rs"]
mod yaml_parser_test;

