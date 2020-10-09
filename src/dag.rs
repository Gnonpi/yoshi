use crate::errors::YoshiError;
use crate::type_definition::NodeId;
use crate::task_node::TaskNode;
use std::collections::HashMap;
use log::{debug, info};
use petgraph::graphmap::DiGraphMap;

/// The set of TaskNode we want to run
struct Dag {
    start_node: Option<NodeId>,
    graph_nodes: DiGraphMap<NodeId, ()>,
    map_nodes: HashMap<NodeId, Box<TaskNode>>
}

impl Dag {
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

    fn new() -> Self {
        Dag {
            start_node: None,
            graph_nodes: DiGraphMap::new(),
            map_nodes: HashMap::new()
        }
    }

    fn get_node(&self, node_id: &NodeId) -> Option<&Box<TaskNode>> {
        self.map_nodes.get(node_id)
    }

    fn contains_node(&self, node_id: &NodeId) -> bool {
        self.map_nodes.contains_key(node_id)
    }

    fn insert_node(&mut self, node: TaskNode, parent_ids: Vec<&NodeId>, children_ids: Vec<&NodeId>) {
        for parent_id in parent_ids.iter() {
            if !self.contains_node(parent_id) {
                panic!("Trying to add node with unexistent parent {}", parent_id.to_string());
            }
        }
        for child_id in children_ids.iter() {
            if !self.contains_node(child_id) {
                panic!("Trying to add node with unexistent child {}", child_id);
            }
        }
        let new_node_id = node.id_node.clone();
        info!("Adding node {}", new_node_id);
        self.graph_nodes.add_node(new_node_id);
        self.map_nodes.insert(new_node_id, Box::new(node));

        for parent_id in parent_ids.iter() {
            self.graph_nodes.add_edge((*parent_id).clone(), new_node_id, ());
        }
        for child_id in children_ids.iter() {
            self.graph_nodes.add_edge(new_node_id, (*child_id).clone(), ());
        }
    }

    fn set_starting_node(&mut self, node_id: NodeId) {
        info!("Setting starting node {}", node_id);
        if !self.contains_node(&node_id) {
            panic!("Cannot set starting unexistent node {}", node_id);
        }
        self.start_node = Some(node_id)
    }

    /*
    fn run(&mut self) -> Result<(), YoshiError> {
        info!("Starting dag");
        let mut bag_of_nodes = vec![self.start_node.clone()];
        let mut bag_of_instances = vec![];

        while bag_of_nodes.len() > 0 {
            if let Some(mut node) = bag_of_nodes.pop() {
                debug!("Treating node {:?}", node.id_node);
                if !node.complete() {
                    // todo: replace with dag runner system
                    node.run();
                }
                match node.instance {
                    Some(task_instance) => {
                        debug!("Storing task instance");
                        bag_of_instances.push(task_instance);
                    }
                    None => {
                        panic!("Complete node with no instance");
                    }
                }

                for child_node in node.children {
                    debug!("Adding child {:?} to things to run", child_node.id_node);
                    bag_of_nodes.push(*child_node);
                }
            }
        }
        info!("Done!");
        Ok(())
    }
    */
}
