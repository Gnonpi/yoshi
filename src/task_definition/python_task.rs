use super::task_def::{generate_task_definition_id, TaskDefinition};
use crate::errors::YoshiError;
use crate::type_definition::{FilePath, TaskId};
use log::{debug, error, info};
use std::collections::HashMap;
use std::process::Command;

/// A Python task that runs a Python script
#[derive(Clone)]
pub struct PythonTaskDefinition {
    // python_bin_path: Filepath
    task_def_id: TaskId,
    script_path: Box<FilePath>,
    args: Vec<String>,
}

impl TaskDefinition for PythonTaskDefinition {
    fn task_definition_id(&self) -> TaskId {
        self.task_def_id
    }

    fn run(&self) -> Result<(), YoshiError> {
        info!(
            "Starting Python script {:?}..{:?}",
            self.script_path, self.args
        );
        let script_path = (*self.script_path).clone();
        let py_command = Command::new("python3")
            .arg(script_path.into_string().unwrap())
            .args(self.args.clone())
            .spawn()
            .expect("failed to execute Python script");
        // todo: stdout is shown but not captured
        let py_result = py_command
            .wait_with_output()
            .expect("failed to wait on Python script");
        debug!("python stdout: {:?}", py_result.stdout);
        if !py_result.status.success() {
            error!("Python script crashed");
            let err = YoshiError {
                message: "Python script was not a success".to_owned(),
                origin: "PythonTaskDefinition::run".to_owned(),
            };
            return Err(err);
        }
        Ok(())
    }

    fn get_params(&self) -> HashMap<String, String> {
        debug!("Getting Python script parameters");
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

impl PythonTaskDefinition {
    fn new(script_path: FilePath, args: Vec<String>) -> Self {
        debug!("Creating Python task definition");
        PythonTaskDefinition {
            task_def_id: generate_task_definition_id(),
            script_path: Box::new(script_path),
            args,
        }
    }
}

#[cfg(test)]
#[path = "./python_task_test.rs"]
mod python_task_test;
