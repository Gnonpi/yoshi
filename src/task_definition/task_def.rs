use crate::errors::YoshiError;
use crate::type_definition::TaskId;
use std::collections::HashMap;

pub trait TaskDefinition {
    fn task_definition_id(&self) -> TaskId;
    fn run(&self) -> Result<(), YoshiError>;
    fn get_params(&self) -> HashMap<String, String>;
}

pub fn generate_task_definition_id() -> TaskId {
    TaskId::new_v4()
}
