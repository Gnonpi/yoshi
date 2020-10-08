use crate::task_node::TaskNode;
use crate::task_instance::TaskStatus;
use crate::task_definition::BashTaskDefinition;
use crate::task_definition::task_def::generate_task_definition_id;
use chrono::prelude::*;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

fn _produce_task_node() -> TaskNode {
    let t_def = BashTaskDefinition::new(vec!["echo".to_owned(), "'Hello'".to_owned()]);
    TaskNode::new(
        Vec::<TaskNode>::new(),
        Box::new(t_def)
    )
}

#[test]
fn it_can_create_new_node() {
    let t_def = BashTaskDefinition::new(vec!["echo".to_owned(), "'Hello'".to_owned()]);
    let new_node = TaskNode::new(
        Vec::<TaskNode>::new(),
        Box::new(t_def)
    );
    assert_eq!(new_node.parents.len(), 0);
    assert_eq!(new_node.children.len(), 0);
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
    assert_eq!(instance.id_task_definition, node.definition.task_definition_id());
    assert_eq!(instance.id_task_runner, node.runner);
    assert_eq!(instance.status, TaskStatus::Success);
    assert!(instance.date_started > time_before);
    assert!(instance.date_finished > time_before);
}

#[test]
fn it_says_complete_after_success() {
    let mut node = _produce_task_node();
    assert!(! node.complete());
    node.run();
    assert!(node.complete());
    match node.instance.as_mut() {
        Some(i) => { i.status = TaskStatus::Failure },
        None => { panic!("Panik") }
    }
    assert!(! node.complete());
}

#[test] #[ignore]
fn it_has_an_output_after_running() {
    // this will have to wait a bit for 
    // task output to be better defined
    assert!(false);
}

#[test]
fn it_can_add_children() {
    let mut parent_node_a = _produce_task_node();
    let mut parent_node_b = _produce_task_node();
    let child_node_c = _produce_task_node();
    let id_a = parent_node_a.id_node;
    let id_b = parent_node_b.id_node;
    let id_c = child_node_c.id_node;

    assert_eq!(parent_node_a.children.len(), 0);
    assert_eq!(parent_node_b.parents.len(), 0);
    assert_eq!(child_node_c.parents.len(), 0);

    parent_node_a.add_child(child_node_c);
    assert_eq!(parent_node_a.children.len(), 1);
    assert_eq!(parent_node_a.children[0].id_node, id_b);
    assert_eq!(child_node_c.parents.len(), 1);
    assert_eq!(child_node_c.parents[0].id_node, id_a);
}
