use crate::dag_parsing::YamlDagConfigParser;
use crate::dag_parsing::dag_config_parser::DagConfigParser;
use crate::dag_parsing::dag_config::DagConfig;
use crate::runners::TaskRunnerType;
use crate::dag::Dag;
use crate::task_node::TaskNode;
use crate::task_definition::{DummyTaskDefinition, PythonTaskDefinition, BashTaskDefinition};
use crate::type_definition::FilePath;
use std::fs;
use log::{debug, info};


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
    // expected dag
    let local_runner_id = TaskRunnerType::LocalBlocking;
    
    let dummy_def = DummyTaskDefinition {};
    let python_def = PythonTaskDefinition::new(
        FilePath::from("/tmp/script.py"),
        vec![]
    );
    let bash_def = BashTaskDefinition::new(vec!["echo 1".to_string()]);

    let nodeA = TaskNode::new(Box::new(python_def), local_runner_id);
    let nodeB = TaskNode::new(Box::new(bash_def), local_runner_id);
    let dummy_start = TaskNode::new(Box::new(dummy_def.clone()), local_runner_id);
    let dummy_end = TaskNode::new(Box::new(dummy_def.clone()), local_runner_id);
    let id_nodeA = nodeA.id_node.clone();
    let id_nodeB = nodeB.id_node.clone();
    let id_dummy_start = dummy_start.id_node.clone();
    let id_dummy_end = dummy_end.id_node.clone();

    let mut expected_dag = Dag::new();

    info!("Creating expected dag");
    debug!("dummy_start");
    expected_dag.add_task(
        dummy_start,
        None,
        None
    );
    debug!("nodeA");
    expected_dag.add_task(
        nodeA,
        Some(vec![&id_dummy_start]),
        None
    );
    debug!("nodeB");
    expected_dag.add_task(
        nodeB,
        Some(vec![&id_dummy_start]),
        None
    );
    debug!("dummy_end");
    expected_dag.add_task(
        dummy_end,
        Some(vec![&id_nodeA, &id_nodeB]),
        None
    );

    // create from config
    let dag_config = create_dag_config();
    let result_dag = Dag::from(dag_config);

    info!("Comparing DAG");
    assert_eq!(expected_dag, result_dag);

    assert!(1 == 2);
}

#[test]
fn test_get_dag_config_from_file() {
    assert!(1 == 2);
}

#[test]
fn test_get_dag_from_file() {
    assert!(1 == 2);
}
