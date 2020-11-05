use crate::dag::Dag;
use crate::type_definition::FilePath;
use crate::dag_parsing::{DagParsingError, DagConfigParser};
use std::collections::HashMap;

type NodeConfigId = String;
type DefinitionConfigId = String;
type RunnerDefinitionId = String;
struct NodeConfig {}
struct DefinitionConfig {}
struct RunnerDefinition {}

pub(super) struct DagConfig {
    nodes: HashMap<NodeConfigId, NodeConfig>,
    definitions: HashMap<DefinitionConfigId, DefinitionConfig>,
    runners: HashMap<RunnerDefinitionId, RunnerDefinition>,
    dag_edges: HashMap<NodeConfigId, Vec<NodeConfigId>>
}

impl DagConfig {
    fn validate(&self) -> bool {

    }
}

// todo: should we add a From or Into trait to DagConfig?
// From<T> for U means Into<U> for T --> one direction
// Into is reflexive

impl From<DagConfig> for Dag {
    fn from(dag_config: DagConfig) -> Self {
        // to impl
    }
}

/// ------------------------------------


fn get_dag_config_from_file(filepath: FilePath, parser: dyn DagConfigParser) -> Result<DagConfig, DagParsingError> {
    // get content of file
    // call validate
    // call parse_file
}

pub fn get_dag_from_file(filepaht: FilePath) -> Result<Dag, DagParsingError> {
    // deduce format from suffix (optional format enum parameter?)
    // match over suffix to use the right parser
    // pass parser and filepath to get_dag_config_from_file
    // return Dag::from the result
}
