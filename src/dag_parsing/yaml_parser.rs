use super::dag_config_parser::{SupportedFormat, DagConfigParser};
use super::dag_config::DagConfig;

pub struct YamlDagConfigParser {}

impl DagConfigParser for YamlDagConfigParser {
    fn get_format(&self) -> SupportedFormat {
        SupportedFormat::Yaml
    }
    
    fn validate(&self, content: String) -> bool {

    }
    
    fn parse_file(&self, content: String) -> DagConfig {

    }
}
