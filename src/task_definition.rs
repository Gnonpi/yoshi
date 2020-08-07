use crate::errors::YoshiError;
use crate::type_definition::FilePath;
use std::collections::HashMap;

pub trait TaskDefinition {
    fn run(&self) -> Result<(), YoshiError>;
    fn get_params(&self) -> HashMap<String, String>;
}

struct PythonTaskDefinition {
    // python_bin_path: Filepath
    script_path: Box<FilePath>,
    args: Vec<String>,
}

impl TaskDefinition for PythonTaskDefinition {
    fn run(&self) -> Result<(), YoshiError> {
        // todo: implement running Python scripts
        println!("Running {:?} -- {:?}", self.script_path, self.args);
        Ok(())
    }

    fn get_params(&self) -> HashMap<String, String> {
        let mut params: HashMap<String, String> = HashMap::new();
        let script_path_copy = (*self.script_path).clone();
        let script_path_string = script_path_copy.into_string().unwrap();
        params.insert("script_path".to_string(), script_path_string);

        let mut arg_string = "".to_string();
        for arg in self.args.iter() {
            arg_string.push_str(&arg);
            arg_string.push_str(" ");
        }
        params.insert("args".to_string(), arg_string);
        params
    }
}

#[cfg(test)]
#[path = "./task_definition_test.rs"]
mod task_definition_test;
