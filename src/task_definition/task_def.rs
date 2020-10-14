use crate::errors::YoshiError;
use crate::task_output::TaskOutput;
use crate::type_definition::TaskId;
use dyn_clone::DynClone;
use std::collections::HashMap;
use std::fmt::Debug;

/// Trait that define a task that can be started
/// basically what's to be done
pub trait TaskDefinition: DynClone + Debug {
    fn task_definition_id(&self) -> TaskId;
    fn run(&self) -> Result<TaskOutput, YoshiError>;
    fn get_params(&self) -> HashMap<String, String>;
}

/// Generate a random (99.99% unique) task id
pub fn generate_task_definition_id() -> TaskId {
    TaskId::new_v4()
}

/// Allow for Sized Box<dyn TaskDefinition>
dyn_clone::clone_trait_object!(TaskDefinition);
