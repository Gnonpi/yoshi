use crate::dag::Dag;
use crate::dag_parsing::{DagConfigParser, DagParsingError, YamlDagConfigParser};
use crate::runners::string_to_runner_type;
use crate::task_definition::{
    string_to_definition_type, DefinitionArguments, TaskDefinition, TaskDefinitionType,
};
use crate::task_node::TaskNode;
use crate::type_definition::{FilePath, NodeId};
use log::{debug, error, info};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// todo: could we do something to avoid repeating those pub(crate)?
pub(crate) type NodeConfigId = String;
pub(crate) type DefinitionConfigId = String;
pub(crate) type RunnerConfigId = String;
#[derive(Debug)]
pub(crate) struct NodeConfig {
    pub(crate) id_node: NodeConfigId,
    pub(crate) ref_definition: DefinitionConfigId,
    pub(crate) ref_runner: RunnerConfigId,
}
#[derive(Debug)]
pub(crate) struct DefinitionConfig {
    pub(crate) id_definition: DefinitionConfigId,
    pub(crate) definition_type: String,
    pub(crate) params: HashMap<String, String>,
}
#[derive(Debug)]
pub(crate) struct RunnerConfig {
    pub(crate) id_runner: RunnerConfigId,
    pub(crate) runner_type: String,
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
    pub(crate) dag_edges: HashMap<NodeConfigId, Vec<NodeConfigId>>,
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
                error!(
                    "{:?} ref to definition {:?} that is not defined",
                    node_name, def_name
                );
                return false;
            }
            if !self.runners.contains_key(&runner_name) {
                error!(
                    "{:?} ref to runner {:?} that is not defined",
                    node_name, runner_name
                );
                return false;
            }
        }
        info!("Checking that all edges point to defined nodes");
        for (parent_node, children_nodes) in self.dag_edges.iter() {
            if !self.nodes.contains_key(parent_node) {
                error!("Parent node {:?} not known", parent_node);
                return false;
            }
            for child_node in children_nodes.iter() {
                if !self.nodes.contains_key(parent_node) {
                    error!(
                        "Child node {:?} referenced by parent {:?} not known",
                        child_node, parent_node
                    );
                    return false;
                }
            }
        }
        true
    }
}

impl From<DagConfig> for Dag {
    /// Create a Dag from a DagConfig
    fn from(dag_config: DagConfig) -> Self {
        info!("Creating DAG from config:\n{:#?}", dag_config);
        let mut node_cfg_id_to_node_id = HashMap::<NodeConfigId, NodeId>::new();
        let mut dag = Dag::new();
        for (node_cfg_id, node_cfg) in dag_config.nodes.iter() {
            // linking def and run
            let def_cfg = &dag_config.definitions[&node_cfg.ref_definition];
            let runner_cfg = &dag_config.runners[&node_cfg.ref_runner];

            // getting enum types
            let def_type = string_to_definition_type(def_cfg.definition_type.clone()).unwrap();
            let runner_type = string_to_runner_type(runner_cfg.runner_type.clone()).unwrap();

            // transforming def params to definition arguments
            let mut da = DefinitionArguments::new();
            for (key, value) in def_cfg.params.iter() {
                da.set(key, value.to_string());
            }

            // creating node
            let mut node = TaskNode::new(def_type, da, runner_type);
            node.set_label(node_cfg_id);
            node_cfg_id_to_node_id.insert(node_cfg_id.to_string(), node.id_node);
            dag.add_task(node, None, None).unwrap();
        }
        // Adding edges
        // normally all nodes are already inserted
        // todo: could those clones be avoided by consuming the map element by element?
        for (parent_id, children_id) in dag_config.dag_edges.iter() {
            let parent_node_id = node_cfg_id_to_node_id.get(parent_id).unwrap();
            for child_id in children_id.iter() {
                let child_node_id = node_cfg_id_to_node_id.get(child_id).unwrap();
                dag.add_edge(*parent_node_id, *child_node_id).unwrap();
            }
        }

        dag
    }
}

/// ------------------------------------

fn get_dag_config_from_file(
    filepath: FilePath,
    parser: &dyn DagConfigParser,
) -> Result<DagConfig, DagParsingError> {
    // get content of file
    let content = fs::read_to_string(filepath).expect("failed to read example file");
    // call validate
    let dag_config = parser.parse_file(content).unwrap();
    // call parse_file
    if !dag_config.validate() {
        return Err(DagParsingError {
            reason: "failed to validate dag config".to_string(),
        });
    }
    Ok(dag_config)
}

pub fn get_dag_from_file(filepath: FilePath) -> Result<Dag, DagParsingError> {
    // deduce format from suffix (optional format enum parameter?)
    let path = filepath.clone().into_string().unwrap();
    let suffix = Path::new(&path).extension().unwrap();
    // match over suffix to use the right parser
    let parser;
    match suffix.to_str().unwrap() {
        "yaml" | "yml" => {
            parser = YamlDagConfigParser {};
        }
        _ => {
            return Err(DagParsingError {
                reason: format!("Unknown suffix: {:?}", suffix),
            })
        }
    }
    // pass parser and filepath to get_dag_config_from_file
    let dag_config = get_dag_config_from_file(filepath, &parser).unwrap();
    let dag = Dag::from(dag_config);

    // return Dag::from the result
    Ok(dag)
}

#[cfg(test)]
#[path = "./dag_config_test.rs"]
mod dag_config_test;
