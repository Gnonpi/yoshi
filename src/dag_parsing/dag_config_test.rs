use crate::dag::Dag;
use crate::dag_parsing::dag_config::DagConfig;
use crate::dag_parsing::dag_config_parser::DagConfigParser;
use crate::dag_parsing::get_dag_from_file;
use crate::dag_parsing::YamlDagConfigParser;
use crate::runners::TaskRunnerType;
use crate::task_definition::{
    BashTaskDefinition, DummyTaskDefinition, PythonTaskDefinition, TaskDefinitionType,
};
use crate::task_node::TaskNode;
use crate::type_definition::{FilePath, NodeId};
use log::{debug, info};
use std::collections::HashMap;
use std::fs;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

fn create_dag_config() -> DagConfig {
    init_logger();
    let example_path = "src/dag_parsing/examples/example2.yaml";
    let content = fs::read_to_string(example_path).expect("failed to read example file");
    let ycp = YamlDagConfigParser {};
    ycp.parse_file(content).unwrap()
}

#[test]
fn it_can_validate_dag_config() {
    let dag_config = create_dag_config();
    assert!(dag_config.validate());
}

#[test]
fn it_can_take_config_to_dag() {
    init_logger();

    // create from config
    let dag_config = create_dag_config();
    let result_dag = Dag::from(dag_config);

    info!("Comparing DAG");
    println!("Result dag: {:#?}", result_dag);

    // start_node is none
    assert!(result_dag.start_nodes.is_empty());
    // created dag has 1 - 2 - 1 nodes
    assert_eq!(result_dag.graph_nodes.node_count(), 4);
    assert_eq!(result_dag.map_nodes.len(), 4);

    // mapping labels to node_id(uuid)
    let mut map_node_id_to_label = HashMap::<String, NodeId>::new();
    for (node_id, node) in result_dag.map_nodes.iter() {
        let node_label = node.label.as_ref().unwrap();
        map_node_id_to_label.insert(node_label.clone(), node.id_node);
    }

    //
    let dummy_start_id = map_node_id_to_label.get("dummy_start").unwrap();
    let dummy_end_id = map_node_id_to_label.get("dummy_end").unwrap();
    let nodeA_id = map_node_id_to_label.get("nodeA").unwrap();
    let nodeB_id = map_node_id_to_label.get("nodeB").unwrap();

    // dummy_start
    let dummy_start_node = result_dag.map_nodes.get(dummy_start_id).unwrap();
    assert_eq!(
        dummy_start_node.definition.task_type(),
        TaskDefinitionType::Dummy
    );
    assert_eq!(dummy_start_node.id_runner, TaskRunnerType::LocalBlocking);
    let neighbors_start: Vec<NodeId> = result_dag.graph_nodes.neighbors(*dummy_start_id).collect();
    assert_eq!(neighbors_start, vec![nodeA_id.clone(), nodeB_id.clone()]);

    // dummy end
    let dummy_end_node = result_dag.map_nodes.get(dummy_end_id).unwrap();
    assert_eq!(
        dummy_end_node.definition.task_type(),
        TaskDefinitionType::Dummy
    );
    assert_eq!(dummy_end_node.id_runner, TaskRunnerType::LocalBlocking);
    let neighbors_end: Vec<NodeId> = result_dag.graph_nodes.neighbors(*dummy_end_id).collect();
    assert_eq!(neighbors_end, vec![]);

    // nodeA
    let nodeA_node = result_dag.map_nodes.get(nodeA_id).unwrap();
    assert_eq!(
        nodeA_node.definition.task_type(),
        TaskDefinitionType::Python
    );
    assert_eq!(nodeA_node.id_runner, TaskRunnerType::LocalBlocking);
    let neighbors_A: Vec<NodeId> = result_dag.graph_nodes.neighbors(*nodeA_id).collect();
    assert_eq!(neighbors_A, vec![dummy_end_id.clone()]);

    // todo: add nodeB
    // todo: check definition params
}

// todo: move to integration test
#[test]
fn test_get_dag_from_file() {
    let example_path = "src/dag_parsing/examples/example2.yaml";
    let dag = get_dag_from_file(FilePath::from(example_path));

    assert!(dag.is_ok());
}
