use crate::errors::YoshiError;
use crate::runners::MessageFromRunner::{Done, Failure};
use crate::runners::TaskRunnerFactory;
use crate::task_node::TaskNode;
use crate::type_definition::NodeId;
use crossbeam_channel::TryRecvError;
use log::{debug, info};
use petgraph::graphmap::DiGraphMap;
use std::collections::HashMap;

// todo: move to its own module
type GraphNodeId = DiGraphMap<NodeId, ()>;

/// The set of TaskNode we want to run
/// Handle the stories of parents/children nodes
#[derive(Debug)]
pub struct Dag {
    pub start_node: Option<NodeId>,
    pub(crate) graph_nodes: GraphNodeId,
    pub(crate) map_nodes: HashMap<NodeId, TaskNode>,
}

impl Dag {
    /// Create a new dag
    pub fn new() -> Self {
        Dag {
            start_node: None,
            graph_nodes: GraphNodeId::new(),
            map_nodes: HashMap::new(),
        }
    }

    /// Get a reference to a node given its id
    pub fn get_node(&self, node_id: &NodeId) -> Option<&TaskNode> {
        self.map_nodes.get(node_id)
    }

    /// Get a mutable reference to a node given its id
    pub fn get_mut_node(&mut self, node_id: &NodeId) -> Option<&mut TaskNode> {
        self.map_nodes.get_mut(node_id)
    }

    /// Whether or not an id refer to a node in the dag
    pub fn contains_node(&self, node_id: &NodeId) -> bool {
        self.map_nodes.contains_key(node_id)
    }

    /// Given the id of a node, if the node is in the graph, returns the children of that node
    pub fn get_children_of_node(&self, node_id: &NodeId) -> Option<Vec<NodeId>> {
        if !self.contains_node(node_id) {
            return None;
        }
        // if graph is directed, neighbors is outgoing nodes
        let neighbors = self.graph_nodes.neighbors(*node_id);
        Some(neighbors.collect())
    }

    /// Add a node to the DAG with possibly the parents and children
    pub fn add_task(
        &mut self,
        node: TaskNode,
        parent_ids: Option<Vec<&NodeId>>,
        children_ids: Option<Vec<&NodeId>>,
    ) {
        //Checking that parents and children ids are present
        // todo: is there a way to do this better?
        if let Some(some_parent_ids) = parent_ids.clone() {
            for parent_id in some_parent_ids.iter() {
                if !self.contains_node(parent_id) {
                    panic!(
                        "Trying to add node with unexistent parent {}",
                        parent_id.to_string()
                    );
                }
            }
        }
        if let Some(some_children_ids) = children_ids.clone() {
            for child_id in some_children_ids.iter() {
                if !self.contains_node(child_id) {
                    panic!("Trying to add node with unexistent child {}", child_id);
                }
            }
        }
        // Adding the node
        let new_node_id = node.id_node;
        info!("Adding node {}", new_node_id);
        self.graph_nodes.add_node(new_node_id);
        self.map_nodes.insert(new_node_id, node);

        // Linking parents and children
        if let Some(some_parent_ids) = parent_ids {
            for parent_id in some_parent_ids.iter() {
                self.graph_nodes.add_edge(*(*parent_id), new_node_id, ());
            }
        }
        if let Some(some_children_ids) = children_ids {
            for child_id in some_children_ids.iter() {
                self.graph_nodes.add_edge(new_node_id, *(*child_id), ());
            }
        }
    }

    // todo: add custom error (same as add_task)
    // todo: use ref ids instead of move
    /// Add an edge between two existing nodes
    pub fn add_edge(&mut self, parent_id: NodeId, child_id: NodeId) {
        if !self.contains_node(&parent_id) {
            panic!(
                "Trying to add edge with unexistent parent {}",
                parent_id.to_string()
            );
        }
        if !self.contains_node(&child_id) {
            panic!(
                "Trying to add edge with unexistent child {}",
                child_id.to_string()
            );
        }
        self.graph_nodes.add_edge(parent_id, child_id, ());
    }

    /// Set the node from which the execution start
    pub fn set_starting_node(&mut self, node_id: NodeId) {
        info!("Setting starting node {}", node_id);
        if !self.contains_node(&node_id) {
            panic!("Cannot set starting unexistent node {}", node_id);
        }
        self.start_node = Some(node_id)
        // todo: if there is start_node when starting to run, find sources nodes
    }

    // shitty implementation first
    /*
    When we mount the Dag,
        it pops up a list of nodes to execute
        then while that list is not empty
        it takes on node and runs it
            when a node is ran
            it checks if it's complete via the output/complete
            if yes, skip and maybe return task_instance
            if not, it runs the task_definition
            when a node is done,
            we add its children to the list
    */
    pub fn run(&mut self) -> Result<(), YoshiError> {
        info!("Starting dag");
        if self.start_node.is_none() {
            // todo: when no starting_node is set, find one candidate then crash
            panic!("Dag cannot start without starting node");
        }
        let mut bag_of_nodes = vec![self.start_node.unwrap().clone()];
        let mut bag_of_instances = vec![];

        // While there are nodes in the bag
        while !bag_of_nodes.is_empty() {
            if let Some(id_node) = bag_of_nodes.pop() {
                let mut node = self.get_mut_node(&id_node).unwrap();
                debug!("Treating node {:?}", node.id_node);
                if !node.complete() {
                    info!("Node {:?} is not complete, running it", node.id_node);

                    let mut node_runner = TaskRunnerFactory::new_runner(&(*node).id_runner);
                    let (_, recv_from_runner) = node_runner.get_channels();
                    node_runner
                        .start_task(id_node, &(*node.definition))
                        .unwrap();

                    // todo: replace with true spawning&waiting
                    for _ in 0..100 {
                        let result_from_channel = recv_from_runner.try_recv();
                        match result_from_channel {
                            Ok(received_msg) => match received_msg {
                                Done {
                                    start_time,
                                    end_time,
                                } => {
                                    info!("Got message that {:?} is done", node);
                                    node.instance = Some(node_runner.get_task_instance().unwrap());
                                    break;
                                }
                                Failure {
                                    start_time,
                                    reason,
                                    failure_time,
                                } => {
                                    panic!("{:?} failed to run", node);
                                }
                                _ => {
                                    debug!("lol");
                                }
                            },
                            Err(err) => match err {
                                TryRecvError::Empty => {
                                    debug!("No message in channel yet");
                                }
                                _ => {
                                    panic!("Error while reading from channel: {:?}", err);
                                }
                            },
                        };
                    }
                }
                // Add the instance to the instance bag
                match node.instance.clone() {
                    Some(task_instance) => {
                        debug!("Storing task instance");
                        bag_of_instances.push(task_instance.clone());
                    }
                    None => {
                        panic!("Complete node with no instance");
                    }
                }

                // Add the children to next bag
                if let Some(children) = self.get_children_of_node(&id_node) {
                    for child_id_node in children {
                        debug!("Adding child {:?} to things to run", child_id_node);
                        bag_of_nodes.push(child_id_node);
                    }
                }
            }
        }
        info!("Done!");
        Ok(())
    }
}

impl Default for Dag {
    fn default() -> Self {
        Self::new()
    }
}

// todo: change to partialeq
fn equal_graph_nodes(self_graph: &GraphNodeId, other_graph: &GraphNodeId) -> bool {
    // check same number of nodes
    debug!("Equal graph nodes: node_count");
    if self_graph.node_count() != other_graph.node_count() {
        return false;
    }
    // check same number of edges
    debug!("Equal graph nodes: edge_count");
    if self_graph.edge_count() != other_graph.edge_count() {
        return false;
    }
    // check every node in self is present in other
    debug!("Equal graph nodes: contains_node");
    for self_node in self_graph.nodes() {
        debug!("{:?}", self_node);
        if !other_graph.contains_node(self_node) {
            return false;
        }
    }
    // check every edge in self is present in other
    debug!("Equal graph nodes: contains_edge");
    for (origin, dest, dir) in self_graph.all_edges() {
        if !other_graph.contains_edge(origin, dest) {
            return false;
        }
    }
    true
}

impl PartialEq for Dag {
    fn eq(&self, other: &Self) -> bool {
        self.start_node == other.start_node
            && equal_graph_nodes(&self.graph_nodes, &other.graph_nodes)
            && self.map_nodes == other.map_nodes
    }
}

#[cfg(test)]
#[path = "./dag_test.rs"]
mod dag_test;
