use crate::dag::Dag;

#[derive(Debug)]
struct DagParsingError {
    reason: String
}

enum SupportedFormat {
    Yaml,
    Toml
}

// todo: should we add a From or Into trait to DagConfig?
// From<T> for U means Into<U> for T --> one direction
// Into is reflexive

struct DagConfig {
    nodes: HashMap<NodeConfigId, NodeConfig>,
    definitions: HashMap<DefinitionConfigId, DefinitionConfig>,
    runners: HashMap<RunnerDefinitionId, RunnerDefinition>,
    dag_edges: HashMap<NodeConfigId, Vec<NodeConfigId>>
}

impl DagConfig {
    fn validate(&self) -> bool {

    }
}

impl From<DagConfig> for Dag {
    fn from(dag_config: DagConfig) -> Self {
        // to impl
    }
}

/*
---------------------------
todo: move to other modules
---------------------------
*/

trait DagConfigParser {
    fn get_format() -> SupportedFormat;
    fn validate(content: String) -> bool;
    fn parse_file(content: String) -> DagConfigParser;
}

struct YamlDagConfigParser {}
struct TomlDagConfigParser {}

fn get_dag_config_from_file(filepath: FilePath, parser: DagConfigParser) -> Result<DagConfig, DagParsingError> {
    // get content of file
    // call validate
    // call parse_file
}

fn get_dag_from_file(filepaht: FilePath) -> Result<Dag, DagParsingError> {
    // deduce format from suffix (optional format enum parameter?)
    // match over suffix to use the right parser
    // pass parser and filepath to get_dag_config_from_file
    // return Dag::from the result
}
