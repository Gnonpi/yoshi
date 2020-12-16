use crate::dag::Dag;
use crate::errors::YoshiError;
use crate::type_definition::NodeId;
use log::info;
use petgraph::algo::is_cyclic_directed;
use petgraph::Direction;

/// todo: https://trello.com/c/nehAyt5u/47-move-graphnodeid-to-its-module
/// Check if there is a cycle in the graph of our Dag
/// A DAG with a cycle is, by definition, wrong
pub(crate) fn check_contains_cycle(dag: &Dag) -> Result<(), YoshiError> {
    /*
    if is_cyclic_directed(dag.graph_nodes) {
        let err_msg = format!("DAG contains a cycle");
        return Err(YoshiError::DagContainsCycle)
    }
    */
    Ok(())
}

/// Find source nodes in Dag
/// Source nodes are nodes without incoming edges, without parents
pub(crate) fn find_source_nodes(dag: &Dag) -> Vec<NodeId> {
    if dag.graph_nodes.node_count() == 0 {
        info!("Dag has no nodes, cannot find sources");
        return vec![];
    }
    if dag.graph_nodes.node_count() == 1 {
        info!("Dag has only one node, considering it as source");
        let id_node = dag.graph_nodes.nodes().next().unwrap();
        return vec![id_node];
    }
    let mut result = vec![];
    for node in dag.graph_nodes.nodes() {
        let incoming_nodes: Vec<NodeId> = dag
            .graph_nodes
            .neighbors_directed(node, Direction::Incoming)
            .collect();
        if incoming_nodes.len() == 0 {
            result.push(node);
        }
    }
    result
}

/// Find sinks nodes in Dag
/// Sink nodes are nodes without outgoing edges, without children
pub(crate) fn find_sink_nodes(dag: &Dag) -> Vec<NodeId> {
    if dag.graph_nodes.node_count() == 0 {
        info!("Dag has no nodes, cannot find sinks");
        return vec![];
    }
    if dag.graph_nodes.node_count() == 1 {
        info!("Dag has only one node, considering it as sink");
        let id_node = dag.graph_nodes.nodes().next().unwrap();
        return vec![id_node];
    }
    let mut result = vec![];
    for node in dag.graph_nodes.nodes() {
        let outgoing_nodes: Vec<NodeId> = dag
            .graph_nodes
            .neighbors_directed(node, Direction::Outgoing)
            .collect();
        if outgoing_nodes.len() == 0 {
            result.push(node);
        }
    }
    result
}

#[cfg(test)]
#[path = "./dag_checker_test.rs"]
mod dag_checker_test;
