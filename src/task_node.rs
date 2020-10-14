use crate::errors::YoshiError;
use crate::task_definition::TaskDefinition;
use crate::task_instance::{TaskInstance, TaskStatus};
use crate::task_output::TaskOutput;
use crate::type_definition::{NodeId, RunnerId};
use crate::task_runner::TaskRunner;
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
    pub runner: Box<dyn TaskRunner>
}

impl TaskNode {
    /// Create a new node
    pub fn new(definition: Box<dyn TaskDefinition>, runner: Box<dyn TaskRunner>) -> Self {
        debug!("Creating task node");
        TaskNode {
            id_node: NodeId::new_v4(),
            definition,
            instance: None,
            runner
        }
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
            return Some(instance.output.clone());
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
        if self.runner.get_runner_id() != other.runner.get_runner_id() {
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
