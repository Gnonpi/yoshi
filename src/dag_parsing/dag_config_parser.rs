use super::dag_config::DagConfig;
use crate::dag_parsing::DagParsingError;

#[derive(Debug, PartialEq)]
pub enum SupportedFormat {
    Yaml,
    Toml,
}

pub trait DagConfigParser {
    /// Get an enum to describe format
    fn get_format(&self) -> SupportedFormat;

    /// Says if the content is a valid <format> data
    /// AND that it follows the right schema for our config
    fn validate(&self, content: &String) -> bool;

    /// Parse string and return
    fn parse_file(&self, content: String) -> Result<DagConfig, DagParsingError>;
}

// i've added the &self in the signatures to quiet the
// Trait cannot be turned into object errors
// it's 00:25 and I'll think about it later
// todo: think this
