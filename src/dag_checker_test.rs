use crate::task_definition::DummyTaskDefinition;
use crate::type_definition::{NodeId, RunnerId};
use crate::task_node::TaskNode;
use crate::dag::Dag;
use crate::dag_checker::{find_source_nodes, find_sink_nodes};

#[test]
fn it_can_find_one_cycle() {
    assert_eq!(1, 2);
}

#[test]
fn it_can_find_indirect_cycle() {
assert_eq!(1, 2);
}

#[test]
fn it_can_find_sources_nodes() {
    let mut dag = Dag::new();
    let res = find_source_nodes(&dag);
    assert_eq!(res.len(), 0);

    let t_def = DummyTaskDefinition {};
    let first_node = TaskNode::new(Box::new(t_def), RunnerId::Fake);
    let first_id = first_node.id_node.clone();
    dag.add_task(first_node, None, None);
    let res = find_source_nodes(&dag);
    assert_eq!(res, vec![first_id]);

    let t_def = DummyTaskDefinition {};
    let second_node = TaskNode::new(Box::new(t_def), RunnerId::Fake);
    dag.add_task(second_node, Some(vec![&first_id]), None);
    let res = find_source_nodes(&dag);
    assert_eq!(res, vec![first_id]);
}

#[test]
fn it_can_find_no_source_nodes() {
    assert_eq!(1, 2);
}

#[test]
fn it_can_find_sink_nodes() {
    let mut dag = Dag::new();
    let res = find_source_nodes(&dag);
    assert_eq!(res.len(), 0);

    let t_def = DummyTaskDefinition {};
    let first_node = TaskNode::new(Box::new(t_def), RunnerId::Fake);
    let first_id = first_node.id_node.clone();
    dag.add_task(first_node, None, None);
    let res = find_sink_nodes(&dag);
    assert_eq!(res, vec![first_id]);

    let t_def = DummyTaskDefinition {};
    let second_node = TaskNode::new(Box::new(t_def), RunnerId::Fake);
    let second_id = second_node.id_node.clone();
    dag.add_task(second_node, Some(vec![&first_id]), None);
    let res = find_sink_nodes(&dag);
    assert_eq!(res, vec![second_id]);
}

#[test]
fn it_can_find_no_sink_nodes() {
    assert_eq!(1, 2);
}
