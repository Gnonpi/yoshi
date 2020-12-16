use super::dag_config::DagConfig;
use super::dag_config_parser::SupportedFormat;
use crate::dag_parsing::DagConfigParser;
use crate::errors::YoshiError;

pub struct TomlDagConfigParser {}

impl DagConfigParser for TomlDagConfigParser {
    fn get_format(&self) -> SupportedFormat {
        SupportedFormat::Toml
    }

    fn validate(&self, content: &String) -> bool {
        false
    }

    fn parse_file(&self, content: String) -> Result<DagConfig, YoshiError> {
        Err(YoshiError::ToImplementSoon)
    }
}
