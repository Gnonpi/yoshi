use crate::task_definition::DefinitionArguments;
use crate::task_definition::{
    BashTaskDefinition, DummyTaskDefinition, PythonTaskDefinition, TaskDefinition,
};

/// Enum identifying the variant of Definition
#[derive(Debug, Clone, PartialEq)]
pub enum TaskDefinitionType {
    Bash,
    Python,
    Dummy,
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
) -> Box<dyn TaskDefinition> {
    match tdt {
        TaskDefinitionType::Bash => {
            let b_def = BashTaskDefinition::from(arguments);
            Box::new(b_def)
        }
        TaskDefinitionType::Python => {
            let p_def = PythonTaskDefinition::from(arguments);
            Box::new(p_def)
        }
        TaskDefinitionType::Dummy => {
            let d_def = DummyTaskDefinition::from(arguments);
            Box::new(d_def)
        }
        _ => {
            panic!("Definition type not linked to TaskDefinition: {:?}", tdt);
        }
    }
}

#[cfg(test)]
#[path = "./definition_factory_test.rs"]
mod definition_factory_test;
