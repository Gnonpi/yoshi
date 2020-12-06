use crate::errors::YoshiError;
use crate::task_output::TaskOutput;
use crate::type_definition::TaskId;
use crate::task_definition::{TaskDefinitionType, DefinitionArguments};
use dyn_clone::DynClone;
use std::collections::HashMap;
use std::fmt::Debug;

/// Trait that define a task that can be started
/// basically what's to be done
pub trait TaskDefinition: DynClone + From<DefinitionArguments> + Debug {
    /// Return a unique id for the definition (instance)
    fn task_definition_id(&self) -> TaskId;
    /// Return an enum to identify the kind of definition
    fn task_type(&self) -> TaskDefinitionType;
    /// Execute the action defined
    fn run(&self) -> Result<TaskOutput, YoshiError>;
    /// Return a view of the parameters that are going to be used
    fn get_params(&self) -> HashMap<String, String>;  // todo: change to DefinitionArguments
}

/// Generate a random (99.99% unique) task id
pub fn generate_task_definition_id() -> TaskId {
    TaskId::new_v4()
}

/// Allow for Sized Box<dyn TaskDefinition>
dyn_clone::clone_trait_object!(TaskDefinition);
