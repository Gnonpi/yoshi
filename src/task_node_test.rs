use crate::task_node::TaskNode;
use crate::task_definition::BashTaskDefinition;
use crate::task_definition::task_def::generate_task_definition_id;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
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
    assert!(false);
}

#[test]
fn it_says_complete_after_success() {
    assert!(false);
}

#[test]
fn it_has_an_output_after_running() {
    assert!(false);
}

#[test]
fn it_can_add_children() {
    assert!(false);
}
