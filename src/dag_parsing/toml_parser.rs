use super::dag_config::DagConfig;
use super::dag_config_parser::{DagConfigParser, SupportedFormat};

pub struct TomlDagConfigParser {}

impl DagConfigParser for TomlDagConfigParser {
    fn get_format(&self) -> SupportedFormat {
        SupportedFormat::Toml
    }

    fn validate(&self, content: &String) -> bool {}

    fn parse_file(&self, content: String) -> DagConfig {}
}
