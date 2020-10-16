use crate::task_definition::{generate_task_definition_id, BashTaskDefinition};
use crate::task_instance::{TaskStatus, TaskInstance};
use crate::task_node::TaskNode;
use crate::task_runner::{TaskRunner, FakeTaskRunner};
use crate::task_output::TaskOutput;
use chrono::prelude::*;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

fn _produce_task_node() -> TaskNode {
    let t_def = BashTaskDefinition::new(vec!["echo".to_owned(), "'Hello'".to_owned()]);
    let t_run = FakeTaskRunner {};
    TaskNode::new(Box::new(t_def), Box::new(t_run))
}

#[test]
fn it_can_create_new_node() {
    let t_def = BashTaskDefinition::new(vec!["echo".to_owned(), "'Hello'".to_owned()]);
    let t_run = FakeTaskRunner {};
    let new_node = TaskNode::new(
        Box::new(t_def),
        Box::new(t_run.clone())
    );
    assert!(new_node.instance.is_none());
    assert_eq!(new_node.runner.get_runner_id(), t_run.get_runner_id());
}

#[test]
fn it_says_complete_after_success() {
    let mut node = _produce_task_node();
    assert!( !node.complete() );
    
    let instance = TaskInstance {
        id_node: node.id_node.clone(),
        id_task_definition: node.definition.task_definition_id().clone(),
        id_task_runner: node.runner.get_runner_id(),
        date_started: Utc::now(),
        date_finished: Utc::now(),
        status: TaskStatus::Success,
        output: TaskOutput::Text("ok".to_string())
    };
    node.instance = Some(instance);
    assert!( node.complete() );
    
    match node.instance.as_mut() {
        Some(i) => i.status = TaskStatus::Failure,
        None => panic!("Panik"),
    }
    assert!( !node.complete() );
}

#[test]
#[ignore]
fn it_has_an_output_after_running() {
    // this will have to wait a bit for
    // task output to be better defined
    assert!(false);
}
