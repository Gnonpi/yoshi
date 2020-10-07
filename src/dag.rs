use crate::errors::YoshiError;
use crate::task_node::TaskNode;
use log::{debug, info};

/// The set of TaskNode we want to run
struct Dag {
    start_node: TaskNode,
}

impl Dag {
    // shitty implementation first
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
                    bag_of_nodes.push(child_node);
                }
            }
        }
        info!("Done!");
        Ok(())
    }
}
