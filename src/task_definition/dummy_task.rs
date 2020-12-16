use crate::errors::YoshiError;
use crate::task_definition::{
    generate_task_definition_id, DefinitionArguments, TaskDefinition, TaskDefinitionType,
};
use crate::task_output::TaskOutput;
use crate::type_definition::TaskId;
use std::collections::HashMap;
use std::convert::TryFrom;

/// A task definition that cannot fail,
/// use to sync
#[derive(Clone, Debug)]
pub struct DummyTaskDefinition {}

impl TryFrom<DefinitionArguments> for DummyTaskDefinition {
    type Error = YoshiError;

    fn try_from(da: DefinitionArguments) -> Result<Self, Self::Error> {
        Ok(DummyTaskDefinition::new())
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
