
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


#[cfg(test)]
#[path = "./definition_factory_test.rs"]
mod definition_factory_test;
