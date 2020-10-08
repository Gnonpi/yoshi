use crate::errors::YoshiError;
use crate::task_definition::TaskDefinition;
use crate::task_instance::{TaskInstance, TaskOutput, TaskStatus};
use crate::type_definition::{NodeId, RunnerId};
use chrono::prelude::*;
use log::{debug, info};
use std::rc::Rc;
use std::cell::RefCell;

/// One node in the DAG
/// Contains the info about its place in the dag
/// as well as the info about the task to do
#[derive(Clone)]
pub struct TaskNode {
    pub id_node: NodeId,
    pub parents: Vec<Rc<RefCell<Box<TaskNode>>>>,
    pub children: Vec<Box<TaskNode>>,
    pub definition: Box<dyn TaskDefinition>,
    pub instance: Option<TaskInstance>,
    pub runner: RunnerId, // todo: implement runner part
}

impl TaskNode {
    fn new(
        children: Vec<Box<TaskNode>>,
        definition: Box<dyn TaskDefinition>,
    ) -> Self {
        debug!("Creating task node");
        TaskNode {
            id_node: NodeId::new_v4(),
            parents: Vec::new(),
            children,
            definition,
            instance: None,
            runner: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), YoshiError> {
        // todo: move datetime handling to its own module
        info!("Starting task node {:?}", self.id_node);
        let date_started = Utc::now();
        let _run_out = self.definition.run().unwrap();
        let date_finished = Utc::now();
        info!(
            "Task node {:?} finished, storing task instance",
            self.id_node
        );
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

    pub fn complete(&self) -> bool {
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

    fn add_child(&mut self, new_child: Box<TaskNode>) {
        debug!(
            "Adding {:?} as child to node {:?}",
            new_child.id_node, self.id_node
        );
        let mut new_child_borrow = *new_child;
        new_child_borrow.parents.push(Box::new(*self));
        self.children.push(new_child);
    }
}

#[cfg(test)]
#[path = "./task_node_test.rs"]
mod task_node_test;
