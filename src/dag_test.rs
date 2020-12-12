use crate::dag::Dag;
use crate::runners::TaskRunnerType;
use crate::task_definition::{generate_task_definition_id, TaskDefinitionType, DefinitionArguments, DefinitionArgumentType};
use crate::task_node::TaskNode;

fn _produce_task_node() -> TaskNode {
    let mut da = DefinitionArguments::new();
    da.set(&"command".to_string(), "[\"echo\", \"'Hello'\"".to_string());
    TaskNode::new(TaskDefinitionType::Bash, da, TaskRunnerType::Fake)
}

#[test]
fn it_can_create_a_dag() {
    let dag = Dag::new();
    assert!(dag.start_nodes.is_empty());
    assert_eq!(dag.graph_nodes.node_count(), 0);
    assert_eq!(dag.graph_nodes.edge_count(), 0);
    assert_eq!(dag.map_nodes.len(), 0);
}

#[test]
fn it_can_add_one_node() {
    let mut dag = Dag::new();
    let task = _produce_task_node();
    dag.add_task(task.clone(), None, None);
    assert_eq!(dag.graph_nodes.node_count(), 1);
    assert!(dag.contains_node(&task.id_node));
    assert!(dag.get_node(&task.id_node).is_some());
    let added_task = dag.get_node(&task.id_node).unwrap();
    assert_eq!(task, *added_task);

    assert_eq!(dag.start_nodes, vec![task.id_node]);
}

#[test]
fn it_can_add_node_with_parents() {
    let mut dag = Dag::new();
    let parent_a = _produce_task_node();
    let parent_b = _produce_task_node();
    let child = _produce_task_node();

    let id_parent_a = parent_a.id_node.clone();
    let id_parent_b = parent_b.id_node.clone();
    let id_child = child.id_node.clone();

    dag.add_task(parent_a, None, None);
    dag.add_task(parent_b, None, None);
    dag.add_task(child, Some(vec![&id_parent_a, &id_parent_b]), None);

    assert_eq!(dag.graph_nodes.node_count(), 3);
    assert_eq!(dag.graph_nodes.edge_count(), 2);
    assert_eq!(dag.map_nodes.len(), 3);

    assert!(dag.contains_node(&id_parent_a));
    assert!(dag.contains_node(&id_parent_b));
    assert!(dag.contains_node(&id_child));

    assert!(dag.graph_nodes.contains_edge(id_parent_a, id_child.clone()));
    assert!(dag.graph_nodes.contains_edge(id_parent_b, id_child.clone()));

    assert_eq!(dag.start_nodes, vec![id_parent_a, id_parent_b]);
}

#[test]
fn it_can_add_node_with_parents_and_children() {
    let mut dag = Dag::new();
    let parent_a = _produce_task_node();
    let parent_b = _produce_task_node();
    let child_c = _produce_task_node();
    let child_d = _produce_task_node();
    let middle_node = _produce_task_node();

    let id_parent_a = parent_a.id_node.clone();
    let id_parent_b = parent_b.id_node.clone();
    let id_child_c = child_c.id_node.clone();
    let id_child_d = child_d.id_node.clone();
    let id_middle = middle_node.id_node.clone();

    /*
    parent_a  parent_b
        middle_node
    child_c  child_d
    */

    dag.add_task(parent_a, None, None);
    dag.add_task(parent_b, None, None);

    dag.add_task(child_c, None, None);
    dag.add_task(child_d, None, None);

    dag.add_task(
        middle_node,
        Some(vec![&id_parent_a, &id_parent_b]),
        Some(vec![&id_child_c, &id_child_d]),
    );

    assert_eq!(dag.graph_nodes.node_count(), 5);
    assert_eq!(dag.graph_nodes.edge_count(), 4);
    assert_eq!(dag.map_nodes.len(), 5);

    assert!(dag.contains_node(&id_parent_a));
    assert!(dag.contains_node(&id_parent_b));
    assert!(dag.contains_node(&id_child_c));
    assert!(dag.contains_node(&id_child_d));
    assert!(dag.contains_node(&id_middle));

    assert!(dag
        .graph_nodes
        .contains_edge(id_parent_a, id_middle.clone()));
    assert!(dag
        .graph_nodes
        .contains_edge(id_parent_b, id_middle.clone()));

    assert!(dag.graph_nodes.contains_edge(id_middle.clone(), id_child_c));
    assert!(dag.graph_nodes.contains_edge(id_middle.clone(), id_child_d));

    assert_eq!(dag.start_nodes, vec![id_parent_a, id_parent_b]);
}

#[test]
fn it_can_add_edge() {
    let mut dag = Dag::new();
    let task_one = _produce_task_node();
    let task_two = _produce_task_node();
    dag.add_task(task_one.clone(), None, None);
    dag.add_task(task_two.clone(), None, None);

    dag.add_edge(task_one.id_node, task_two.id_node);
    assert!(dag
        .graph_nodes
        .contains_edge(task_one.id_node, task_two.id_node));
    assert_eq!(dag.start_nodes, vec![task_one.id_node]);
}

#[test]
fn it_can_get_node() {
    let mut dag = Dag::new();
    let task = _produce_task_node();
    dag.add_task(task.clone(), None, None);

    let unknown_id_node = generate_task_definition_id();
    let no_node = dag.get_node(&unknown_id_node);
    assert!(no_node.is_none());

    // retrieve inserted node
    let got_node = dag.get_node(&task.id_node);
    assert!(got_node.is_some());
    let got_node = got_node.unwrap().clone();
    assert_eq!(got_node, task);
}

#[test]
fn it_knows_it_contain_node() {
    let mut dag = Dag::new();
    let task = _produce_task_node();
    dag.add_task(task.clone(), None, None);

    let unknown_node_id = generate_task_definition_id();
    assert!(!dag.contains_node(&unknown_node_id));
    assert!(dag.contains_node(&task.id_node));
}
