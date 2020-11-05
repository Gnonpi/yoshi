use super::dag_config::DagConfig;

pub enum SupportedFormat {
    Yaml,
    Toml
}

pub trait DagConfigParser {
    fn get_format(&self) -> SupportedFormat;
    fn validate(&self, content: String) -> bool;
    fn parse_file(&self, content: String) -> DagConfig;
}

// i've added the &self in the signatures to quiet the 
// Trait cannot be turned into object errors
// it's 00:25 and I'll think about it later
// todo: think this