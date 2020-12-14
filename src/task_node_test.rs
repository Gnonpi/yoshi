use crate::runners::TaskRunnerType;
use crate::task_definition::{DefinitionArguments, TaskDefinitionType};
use crate::task_instance::{TaskInstance, TaskStatus};
use crate::task_node::TaskNode;
use crate::task_output::TaskOutput;
use crate::type_definition::TaskId;
use chrono::prelude::*;

fn _produce_task_node() -> TaskNode {
    let mut da = DefinitionArguments::new();
    da.set(&"command".to_string(), "[\"echo\", \"'Hello'\"".to_string());
    TaskNode::new(TaskDefinitionType::Bash, da, TaskRunnerType::Fake)
}

#[test]
fn it_can_create_new_node() {
    let mut da = DefinitionArguments::new();
    da.set(&"command".to_string(), "[\"echo\", \"'Hello'\"".to_string());
    let new_node = TaskNode::new(TaskDefinitionType::Bash, da, TaskRunnerType::Fake);
    assert!(new_node.instance.is_none());
    assert_eq!(new_node.id_runner, TaskRunnerType::Fake);
}

#[test]
fn it_says_complete_after_success() {
    let mut node = _produce_task_node();
    assert!(!node.complete());

    let instance = TaskInstance {
        id_node: node.id_node.clone(),
        id_task_definition: TaskId::new_v4(),
        id_task_runner: node.id_runner,
        date_started: Utc::now(),
        date_finished: Utc::now(),
        status: TaskStatus::Success,
        output: TaskOutput::Text("ok".to_string()),
    };
    node.instance = Some(instance);
    assert!(node.complete());

    match node.instance.as_mut() {
        Some(i) => i.status = TaskStatus::Failure,
        None => panic!("Panik"),
    }
    assert!(!node.complete());
}

#[test]
fn it_can_add_a_label() {
    let mut node = _produce_task_node();
    assert!(node.label.is_none());
    let label = String::from("cool label");
    node.set_label(&label);
    assert!(node.label.is_some());
    assert_eq!(node.label.unwrap(), label);
}

#[test]
#[ignore]
fn it_has_an_output_after_running() {
    // this will have to wait a bit for
    // task output to be better defined
    assert!(false);
}
