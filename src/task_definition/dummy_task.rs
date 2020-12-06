use crate::errors::YoshiError;
use crate::task_definition::{generate_task_definition_id, TaskDefinition, TaskDefinitionType, DefinitionArguments};
use crate::task_output::TaskOutput;
use crate::type_definition::TaskId;
use std::collections::HashMap;

/// A task definition that cannot fail,
/// use to sync
#[derive(Clone, Debug)]
pub struct DummyTaskDefinition {}

impl From<DefinitionArguments> for DummyTaskDefinition {
    fn from(da: DefinitionArguments) -> Self {
        DummyTaskDefinition::new()
    }
}

impl TaskDefinition for DummyTaskDefinition {
    fn task_definition_id(&self) -> TaskId {
        generate_task_definition_id()
    }

    fn task_type(&self) -> TaskDefinitionType {
        TaskDefinitionType::Dummy
    }

    fn run(&self) -> Result<TaskOutput, YoshiError> {
        Ok(TaskOutput::Nothing)
    }

    fn get_params(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

impl DummyTaskDefinition {
    pub fn new() -> Self {
        DummyTaskDefinition {}
    }
}
