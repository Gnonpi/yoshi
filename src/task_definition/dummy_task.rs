use crate::errors::YoshiError;
use crate::task_definition::{generate_task_definition_id, TaskDefinition};
use crate::task_output::TaskOutput;
use crate::type_definition::TaskId;
use std::collections::HashMap;

/// A task definition that cannot fail,
/// use to sync
#[derive(Clone, Debug)]
pub struct DummyTaskDefinition {}

impl TaskDefinition for DummyTaskDefinition {
    fn task_definition_id(&self) -> TaskId {
        generate_task_definition_id()
    }
    fn run(&self) -> Result<TaskOutput, YoshiError> {
        Ok(TaskOutput::Nothing)
    }

    fn get_params(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}
