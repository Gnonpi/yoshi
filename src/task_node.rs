use crate::errors::YoshiError;
use crate::task_definition::TaskDefinition;
use crate::task_instance::{TaskInstance, TaskOutput, TaskStatus};
use crate::type_definition::{NodeId, RunnerId};

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

    fn run(&self) -> Result<(), YoshiError> {
        self.definition.run()
    }

    fn complete(&self) -> bool {
        if let Some(instance) = &self.instance {
            return instance.status == TaskStatus::Success;
        }
        false
    }

    fn output(&self) -> Option<TaskOutput> {
        if let Some(instance) = &self.instance {
            return Some(instance.output.clone());
        }
        None
    }
}

struct Dag {
    start_node: Box<TaskNode>,
}
