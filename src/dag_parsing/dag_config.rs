use crate::dag::Dag;
use crate::type_definition::FilePath;
use crate::dag_parsing::{DagParsingError, DagConfigParser};
use std::collections::HashMap;
use log::{debug, info, error};

// todo: could we do something to avoid repeating those pub(crate)?
pub(crate) type NodeConfigId = String;
pub(crate) type DefinitionConfigId = String;
pub(crate) type RunnerConfigId = String;
#[derive(Debug)]
pub(crate) struct NodeConfig {
    pub(crate) id_node: NodeConfigId, 
    pub(crate) ref_definition: DefinitionConfigId,
    pub(crate) ref_runner: RunnerConfigId
}
#[derive(Debug)]
pub(crate) struct DefinitionConfig {
    pub(crate) id_definition: DefinitionConfigId,
    pub(crate) definition_type: String,
    pub(crate) params: HashMap<String, String>
}
#[derive(Debug)]
pub(crate) struct RunnerConfig {
    pub(crate) id_runner: RunnerConfigId,
    pub(crate) runner_type: String
}

// todo: should be pub(crate)?
// it's pub here but not referenced in mod.rs
// and only loaded via super::dag_config::DagConfig
/// Intermediate form creaed from file parser
/// that is then turned into a usable DAG
#[derive(Debug)]
pub struct DagConfig {
    pub(crate) nodes: HashMap<NodeConfigId, NodeConfig>,
    pub(crate) definitions: HashMap<DefinitionConfigId, DefinitionConfig>,
    pub(crate) runners: HashMap<RunnerConfigId, RunnerConfig>,
    pub(crate) dag_edges: HashMap<NodeConfigId, Vec<NodeConfigId>>
}

impl DagConfig {
    /// Validate the configuration
    fn validate(&self) -> bool {
        debug!("{:#?}", self);
        info!("Checking that all definition and runner referenced are defined");
        for (node_name, node_cfg) in self.nodes.iter() {
            let def_name = node_cfg.ref_definition.clone();
            let runner_name = node_cfg.ref_runner.clone();
            if !self.definitions.contains_key(&def_name) {
                error!("{:?} ref to definition {:?} that is not defined", node_name, def_name);
                return false
            }
            if !self.runners.contains_key(&runner_name) {
                error!("{:?} ref to runner {:?} that is not defined", node_name, runner_name);
                return false
            }
        }
        info!("Checking that all edges point to defined nodes");
        for (parent_node, children_nodes) in self.dag_edges.iter() {
            if !self.nodes.contains_key(parent_node) {
                error!("Parent node {:?} not known", parent_node);
                return false
            }
            for child_node in children_nodes.iter() {
                if !self.nodes.contains_key(parent_node) {
                    error!("Child node {:?} referenced by parent {:?} not known", child_node, parent_node);
                    return false
                }
            }
        }
        true
    }
}

// todo: should we add a From or Into trait to DagConfig?
// From<T> for U means Into<U> for T --> one direction
// Into is reflexive

impl From<DagConfig> for Dag {
    fn from(dag_config: DagConfig) -> Self {
        info!("Creating DAG from config:\n{:#?}", dag_config);
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


#[cfg(test)]
#[path = "./dag_config_test.rs"]
mod dag_config_test;
