use crate::errors::YoshiError;
use crate::task_definition::TaskDefinition;
use crate::task_instance::{TaskInstance, TaskOutput, TaskStatus};
use crate::type_definition::{NodeId, RunnerId};
use chrono::prelude::*;

struct TaskNode {
    id_node: NodeId,
    parents: Vec<Box<TaskNode>>,
    children: Vec<Box<TaskNode>>,
    definition: Box<dyn TaskDefinition>,
    instance: Option<TaskInstance>,
    runner: RunnerId, // todo: implement runner part
}

impl TaskNode {
    fn new(
        parents: Vec<Box<TaskNode>>,
        children: Vec<Box<TaskNode>>,
        definition: Box<dyn TaskDefinition>,
    ) -> Self {
        TaskNode {
            id_node: 0, // todo: implement uuid
            parents,
            children,
            definition,
            instance: None,
            runner: 0,
        }
    }

    fn run(&mut self) -> Result<(), YoshiError> {
        // todo: move datetime handling to its own module
        let date_started = Utc::now();
        let run_out = self.definition.run().unwrap();
        let date_finished = Utc::now();
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
}

struct Dag {
    start_node: Box<TaskNode>,
}
