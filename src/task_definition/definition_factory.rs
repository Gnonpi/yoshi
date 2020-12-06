use crate::type_definition::FilePath;
use crate::task_definition::{
    DummyTaskDefinition, 
    BashTaskDefinition, 
    PythonTaskDefinition, 
    TaskDefinition
};

/// Enum identifying the variant of Definition
#[derive(Debug, PartialEq)]
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

pub struct DefinitionArguments;

impl DefinitionArguments {
    fn get(key: str) -> String {
        String::from("to impl")
    }
}

struct DefinitionFactory;

impl DefinitionFactory {
    fn new_definition(tdt: &TaskDefinitionType, arguments: &DefinitionArguments) -> Box<dyn TaskDefinition> {
        match tdt {
            TaskDefinitionType::Bash => {
                let b_def = BashTaskDefinition::new(vec!["echo 'Hello'".to_string()]);
                Box::new(b_def)
            },
            TaskDefinitionType::Python => {
                let script_path = FilePath::from("script.py");
                let p_def = PythonTaskDefinition::new(script_path, vec![]);
                Box::new(p_def)
            },
            TaskDefinitionType::Dummy => {
                let d_def = DummyTaskDefinition::new();
                Box::new(d_def)
            },
            _ => {
                panic!("Definition type not linked to TaskDefinition: {:?}", tdt);
            }
        }
    }
}

#[cfg(test)]
#[path = "./definition_factory_test.rs"]
mod definition_factory_test;
