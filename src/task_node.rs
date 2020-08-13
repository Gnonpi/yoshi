use crate::errors::YoshiError;
use crate::task_definition::TaskDefinition;
use crate::task_instance::{TaskInstance, TaskOutput, TaskStatus};
use crate::type_definition::{NodeId, RunnerId};
use chrono::prelude::*;
use log::{debug, info};

/// One node in the DAG
/// Contains the info about its place in the dag
/// as well as the info about the task to do
#[derive(Clone)]
struct TaskNode {
    id_node: NodeId,
    parents: Vec<Box<TaskNode>>,
    children: Vec<TaskNode>,
    definition: Box<dyn TaskDefinition>,
    instance: Option<TaskInstance>,
    runner: RunnerId, // todo: implement runner part
}

impl TaskNode {
    fn new(
        parents: Vec<Box<TaskNode>>,
        children: Vec<TaskNode>,
        definition: Box<dyn TaskDefinition>,
    ) -> Self {
        debug!("Creating task node");
        TaskNode {
            id_node: NodeId::new_v4(),
            parents,
            children,
            definition,
            instance: None,
            runner: 0,
        }
    }

    fn run(&mut self) -> Result<(), YoshiError> {
        // todo: move datetime handling to its own module
        info!("Starting task node {:?}", self.id_node);
        let date_started = Utc::now();
        let _run_out = self.definition.run().unwrap();
        let date_finished = Utc::now();
        info!("Task node {:?} finished, storing task instance", self.id_node);
        let instance = TaskInstance {
            id_task_definition: self.definition.task_definition_id(),
            id_task_runner: self.runner,
            date_started,
            date_finished,
            status: TaskStatus::Success,
            got_output: TaskOutput::Text("ok".to_string()),
        };
        self.instance = Some(instance);
        Ok(())
    }

    fn complete(&self) -> bool {
        debug!("Checking if task node {:?} is complete", self.id_node);
        if let Some(instance) = &self.instance {
            return instance.status == TaskStatus::Success;
        }
        false
    }

    fn output(&self) -> Option<TaskOutput> {
        if let Some(instance) = &self.instance {
            return Some(instance.got_output.clone());
        }
        None
    }

    fn add_child(&mut self, new_child: TaskNode) {
        debug!("Adding {:?} as child to node {:?}", new_child.id_node, self.id_node);
        self.children.push(new_child)
    }
}

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
