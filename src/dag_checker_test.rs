use crate::dag::Dag;
use crate::dag_checker::{check_contains_cycle, find_sink_nodes, find_source_nodes};
use crate::task_definition::{DefinitionArguments, TaskDefinitionType};
use crate::task_node::TaskNode;
use crate::type_definition::RunnerId;

/*
#[test]
fn it_can_find_one_cycle() {
    let mut dag = Dag::new();

    let first_node = TaskNode::new(Box::new(DummyTaskDefinition {}), RunnerId::Fake);
    let second_node = TaskNode::new(Box::new(DummyTaskDefinition {}), RunnerId::Fake);
    dag.add_task(first_node.clone(), None, None);
    // todo: dag.add_task and dag.add_edge should use check_contains_cycle to prevent cycles
    dag.add_task(second_node, Some(vec![&first_node.id_node]), Some(vec![&first_node.id_node]));

    let res = check_contains_cycle(&dag);
    assert!(res.is_err());
}

#[test]
fn it_can_find_indirect_cycle() {
assert_eq!(1, 2);
}
*/

#[test]
fn it_can_find_sources_nodes() {
    let mut dag = Dag::new();
    let res = find_source_nodes(&dag);
    assert_eq!(res.len(), 0);

    let first_node = TaskNode::new(
        TaskDefinitionType::Dummy,
        DefinitionArguments::new(),
        RunnerId::Fake,
    );
    let first_id = first_node.id_node.clone();
    dag.add_task(first_node, None, None).unwrap();
    let res = find_source_nodes(&dag);
    assert_eq!(res, vec![first_id]);

    let second_node = TaskNode::new(
        TaskDefinitionType::Dummy,
        DefinitionArguments::new(),
        RunnerId::Fake,
    );
    dag.add_task(second_node, Some(vec![&first_id]), None)
        .unwrap();
    let res = find_source_nodes(&dag);
    assert_eq!(res, vec![first_id]);
}

#[test]
fn it_can_find_sink_nodes() {
    let mut dag = Dag::new();
    let res = find_source_nodes(&dag);
    assert_eq!(res.len(), 0);

    let first_node = TaskNode::new(
        TaskDefinitionType::Dummy,
        DefinitionArguments::new(),
        RunnerId::Fake,
    );
    let first_id = first_node.id_node.clone();
    dag.add_task(first_node, None, None).unwrap();
    let res = find_sink_nodes(&dag);
    assert_eq!(res, vec![first_id]);

    let second_node = TaskNode::new(
        TaskDefinitionType::Dummy,
        DefinitionArguments::new(),
        RunnerId::Fake,
    );
    let second_id = second_node.id_node.clone();
    dag.add_task(second_node, Some(vec![&first_id]), None)
        .unwrap();
    let res = find_sink_nodes(&dag);
    assert_eq!(res, vec![second_id]);
}
