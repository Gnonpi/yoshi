use crate::errors::YoshiError;
use crate::task_definition::TaskDefinition;
use crate::task_instance::{TaskInstance, TaskStatus};
use crate::task_output::TaskOutput;
use crate::type_definition::{NodeId, RunnerId};
use chrono::prelude::*;
use log::{debug, info};

/// One node in the DAG
/// Contains only the info about the linked task
/// And how to execute it
#[derive(Clone, Debug)]
pub struct TaskNode {
    pub id_node: NodeId,
    pub definition: Box<dyn TaskDefinition>,
    pub instance: Option<TaskInstance>,
    pub runner: RunnerId, // todo: implement runner part
}

impl TaskNode {
    /// Create a new node
    pub fn new(definition: Box<dyn TaskDefinition>) -> Self {
        debug!("Creating task node");
        TaskNode {
            id_node: NodeId::new_v4(),
            definition,
            instance: None,
            runner: 0,
        }
    }

    /// Run the task
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
            id_node: self.id_node.clone(),
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

    /// Can we build on this task?
    pub fn complete(&self) -> bool {
        debug!("Checking if task node {:?} is complete", self.id_node);
        if let Some(instance) = &self.instance {
            return instance.status == TaskStatus::Success;
        }
        false
    }

    /// Once the task has run, this return the output if any
    fn output(&self) -> Option<TaskOutput> {
        if let Some(instance) = &self.instance {
            return Some(instance.got_output.clone());
        }
        None
    }
}

impl PartialEq for TaskNode {
    fn eq(&self, other: &Self) -> bool {
        if self.id_node != other.id_node {
            return false;
        }
        let mut comp_instance = false;
        match (self.instance.as_ref(), other.instance.as_ref()) {
            (Some(lhs), Some(rhs)) => comp_instance = lhs == rhs,
            (None, None) => comp_instance = true,
            _ => comp_instance = false,
        }
        if comp_instance == false {
            return false;
        }
        if self.runner != other.runner {
            return false;
        }
        // todo: not the best thing but it'll have to do
        // i tried implementing a partialeq on taskdefinition
        // but it made problems about
        // trait into objects, not sized, not the right types
        return self.definition.get_params() == other.definition.get_params();
    }
}

#[cfg(test)]
#[path = "./task_node_test.rs"]
mod task_node_test;
