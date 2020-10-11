use crate::task_definition::{BashTaskDefinition, generate_task_definition_id};
use crate::task_instance::TaskStatus;
use crate::task_node::TaskNode;
use chrono::prelude::*;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

fn _produce_task_node() -> TaskNode {
    let t_def = BashTaskDefinition::new(vec!["echo".to_owned(), "'Hello'".to_owned()]);
    TaskNode::new(Box::new(t_def))
}

#[test]
fn it_can_create_new_node() {
    let t_def = BashTaskDefinition::new(vec!["echo".to_owned(), "'Hello'".to_owned()]);
    let new_node = TaskNode::new(Box::new(t_def));
    assert!(new_node.instance.is_none());
    assert_eq!(new_node.runner, 0);
}

#[test]
fn it_can_simply_run() {
    let mut node = _produce_task_node();
    // for now it's sequential
    let time_before = Utc::now();
    let run_result = node.run();
    assert!(run_result.is_ok());
    assert!(node.instance.is_some());

    let instance = node.instance.unwrap();
    assert_eq!(
        instance.id_task_definition,
        node.definition.task_definition_id()
    );
    assert_eq!(instance.id_task_runner, node.runner);
    assert_eq!(instance.status, TaskStatus::Success);
    assert!(instance.date_started > time_before);
    assert!(instance.date_finished > time_before);
}

#[test]
fn it_says_complete_after_success() {
    let mut node = _produce_task_node();
    assert!(!node.complete());
    node.run();
    assert!(node.complete());
    match node.instance.as_mut() {
        Some(i) => i.status = TaskStatus::Failure,
        None => panic!("Panik"),
    }
    assert!(!node.complete());
}

#[test]
#[ignore]
fn it_has_an_output_after_running() {
    // this will have to wait a bit for
    // task output to be better defined
    assert!(false);
}
