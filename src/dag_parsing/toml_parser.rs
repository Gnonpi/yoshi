use super::dag_config_parser::{SupportedFormat, DagConfigParser};
use super::dag_config::DagConfig;

pub struct TomlDagConfigParser {}

impl DagConfigParser for TomlDagConfigParser {
    fn get_format(&self) -> SupportedFormat {
        SupportedFormat::Toml
    }
    
    fn validate(&self, content: String) -> bool {

    }

    fn parse_file(&self, content: String) -> DagConfig {

    }
}
