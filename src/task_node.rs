use crate::task_definition::{DefinitionArguments, TaskDefinitionType};
use crate::task_instance::{TaskInstance, TaskStatus};
use crate::task_output::TaskOutput;
use crate::type_definition::{NodeId, RunnerId};
use log::debug;

/// One node in the DAG
/// Contains only the info about the linked task
/// And how to execute it
#[derive(Clone, Debug)]
pub struct TaskNode {
    pub id_node: NodeId,
    pub label: Option<String>,
    pub definition_type: TaskDefinitionType,
    pub definition_arguments: DefinitionArguments,
    pub instance: Option<TaskInstance>,
    pub id_runner: RunnerId,
}

impl TaskNode {
    /// Create a new node
    pub fn new(
        definition_type: TaskDefinitionType,
        definition_arguments: DefinitionArguments,
        id_runner: RunnerId,
    ) -> Self {
        debug!(
            "Creating task node {:?}-{:?}",
            definition_type.clone(),
            id_runner
        );
        TaskNode {
            id_node: NodeId::new_v4(),
            label: None,
            definition_type,
            definition_arguments,
            instance: None,
            id_runner,
        }
    }

    /// Set a label (human readable name) to the node
    pub fn set_label(&mut self, new_label: &String) {
        self.label = Some(new_label.to_string())
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
        if !comp_instance {
            return false;
        }
        if self.id_runner != other.id_runner {
            return false;
        }
        if self.definition_type != other.definition_type {
            return false;
        }
        return self.definition_arguments == other.definition_arguments;
    }
}

#[cfg(test)]
#[path = "./task_node_test.rs"]
mod task_node_test;
