use crate::type_definition::NodeId;
use crate::errors::YoshiError;
use crate::dag::Dag;
use petgraph::algo::{is_cyclic_directed};

fn check_cycle(dag: &Dag) -> Result<(), YoshiError> {
    /*
    if is_cyclic_directed(dag.graph_nodes) {
        let err_msg = format!("DAG contains a cycle");
        return Err(YoshiError {
            message: err_msg,
            origin: format!("dag_checker:check_cycle")
        })
    }
    Ok(())
    */
    Err(YoshiError {
        message: format!("to impl"),
        origin: format!("dag_checker")
    })
}

fn find_source_nodes(dag: &Dag) -> Vec<NodeId> {
    vec![]
}

fn find_sink_nodes(dag: &Dag) -> Vec<NodeId> {
    vec![]
}

