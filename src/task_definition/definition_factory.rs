use crate::errors::YoshiError;
use crate::task_definition::DefinitionArguments;
use crate::task_definition::{
    BashTaskDefinition, DummyTaskDefinition, PythonTaskDefinition, TaskDefinition,
};
use std::convert::TryFrom;

/// Enum identifying the variant of Definition
#[derive(Debug, Clone, PartialEq)]
pub enum TaskDefinitionType {
    Bash,
    Python,
    Dummy,
    Postgressql,
    Sqlite,
}

/// Given a string, return an enum that link to a definition variant
pub fn string_to_definition_type(def_name: String) -> Option<TaskDefinitionType> {
    match def_name.as_str() {
        "python_task_definition" => Some(TaskDefinitionType::Python),
        "bash_task_definition" => Some(TaskDefinitionType::Bash),
        "dummy_task_definition" => Some(TaskDefinitionType::Dummy),
        _ => None,
    }
}

/// Given a type of task and the arguments to pass it, create a new instance
pub fn create_new_definition(
    tdt: &TaskDefinitionType,
    arguments: DefinitionArguments,
) -> Result<Box<dyn TaskDefinition>, YoshiError> {
    // todo: could this match be replaced by const hashmap?
    match tdt {
        TaskDefinitionType::Bash => {
            // todo: raise YoshiError from here?
            let b_def = BashTaskDefinition::try_from(arguments).unwrap();
            Ok(Box::new(b_def))
        }
        TaskDefinitionType::Python => {
            let p_def = PythonTaskDefinition::try_from(arguments).unwrap();
            Ok(Box::new(p_def))
        }
        TaskDefinitionType::Dummy => {
            let d_def = DummyTaskDefinition::try_from(arguments).unwrap();
            Ok(Box::new(d_def))
        }
        _ => Err(YoshiError::UnlinkedDefinitionType(tdt.clone())),
    }
}

#[cfg(test)]
#[path = "./definition_factory_test.rs"]
mod definition_factory_test;
