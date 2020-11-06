mod dag_config;
mod dag_config_parser;
mod errors;

pub(super) use dag_config_parser::DagConfigParser;

pub use errors::DagParsingError;
pub(super) use dag_config_parser::SupportedFormat;
pub use dag_config::get_dag_from_file;

// could this be in a block in a mod?
#[cfg(feature="toml_parse")]
mod toml_parser;
#[cfg(feature="toml_parse")]
pub(super) use toml_parser::TomlDagConfigParser;

#[cfg(feature="yaml_parse")]
mod yaml_parser;
#[cfg(feature="yaml_parse")]
pub(super) use yaml_parser::YamlDagConfigParser;
