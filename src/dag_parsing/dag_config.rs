use crate::dag::Dag;
use crate::type_definition::FilePath;
use crate::dag_parsing::{DagParsingError, DagConfigParser};
use std::collections::HashMap;

// todo: could we do something to avoid repeating those pub(crate)?
pub(crate) type NodeConfigId = String;
pub(crate) type DefinitionConfigId = String;
pub(crate) type RunnerConfigId = String;
pub(crate) struct NodeConfig {
    pub(crate) id_node: NodeConfigId, 
    pub(crate) ref_definition: DefinitionConfigId,
    pub(crate) ref_runner: RunnerConfigId
}
pub(crate) struct DefinitionConfig {
    pub(crate) id_definition: DefinitionConfigId,
    pub(crate) definition_type: String,
    pub(crate) params: HashMap<String, String>
}
pub(crate) struct RunnerConfig {
    pub(crate) id_runner: RunnerConfigId,
    pub(crate) runner_type: String
}

// todo: should be pub(crate)?
// it's pub here but not referenced in mod.rs
// and only loaded via super::dag_config::DagConfig
pub struct DagConfig {
    pub(crate) nodes: HashMap<NodeConfigId, NodeConfig>,
    pub(crate) definitions: HashMap<DefinitionConfigId, DefinitionConfig>,
    pub(crate) runners: HashMap<RunnerConfigId, RunnerConfig>,
    pub(crate) dag_edges: HashMap<NodeConfigId, Vec<NodeConfigId>>
}

impl DagConfig {
    fn validate(&self) -> bool {
        // to impl
        false
    }
}

// todo: should we add a From or Into trait to DagConfig?
// From<T> for U means Into<U> for T --> one direction
// Into is reflexive

impl From<DagConfig> for Dag {
    fn from(dag_config: DagConfig) -> Self {
        // to impl
        Dag::new()
    }
}

/// ------------------------------------


fn get_dag_config_from_file(filepath: FilePath, parser: &dyn DagConfigParser) -> Result<DagConfig, DagParsingError> {
    // get content of file
    // call validate
    // call parse_file
    
    // to impl
    Err(DagParsingError {reason: String::from("to impl")})
}

pub fn get_dag_from_file(filepath: FilePath) -> Result<Dag, DagParsingError> {
    // deduce format from suffix (optional format enum parameter?)
    // match over suffix to use the right parser
    // pass parser and filepath to get_dag_config_from_file
    // return Dag::from the result
    
    // to impl
    Err(DagParsingError {reason: String::from("to impl")})
}
