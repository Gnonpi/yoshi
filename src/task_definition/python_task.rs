use crate::errors::YoshiError;
use crate::task_definition::{generate_task_definition_id, TaskDefinition, TaskDefinitionType, DefinitionArguments};
use crate::task_output::TaskOutput;
use crate::type_definition::{FilePath, TaskId};
use log::{debug, error, info};
use std::collections::HashMap;
use std::process::Command;
use std::str;

/// A Python task that runs a Python script
#[derive(Clone, Debug)]
pub struct PythonTaskDefinition {
    // python_bin_path: Filepath
    task_def_id: TaskId,
    script_path: Box<FilePath>,
    args: Vec<String>,
}

impl From<DefinitionArguments> for PythonTaskDefinition {
    fn from(da: DefinitionArguments) -> Self {
        let script_path = da.params.get("script_path").unwrap();
        let args_string = da.params.get("args").unwrap();
        let mut args = vec![];
        if args_string.to_string() == "[]".to_string() {
            let args = vec![];
        } else {
            let args = vec![args_string.to_string()];
        }
        PythonTaskDefinition::new(FilePath::from(script_path), args)
    }
}

impl TaskDefinition for PythonTaskDefinition {
    fn task_definition_id(&self) -> TaskId {
        self.task_def_id
    }

    fn task_type(&self) -> TaskDefinitionType {
        TaskDefinitionType::Python
    }

    fn run(&self) -> Result<TaskOutput, YoshiError> {
        info!(
            "Starting Python script {:?}..{:?}",
            self.script_path, self.args
        );
        let script_path = (*self.script_path).clone();
        let py_proc = Command::new("python3")
            .arg(script_path.into_string().unwrap())
            .args(self.args.clone())
            .output();
        match py_proc {
            Ok(py_result) => {
                if !py_result.status.success() {
                    error!("Python started running but crashed");
                    error!("stderr: {:?}", str::from_utf8(&py_result.stderr));
                    let err = YoshiError {
                        message: "Python script was not a success".to_owned(),
                        origin: "PythonTaskDefinition::run".to_owned(),
                    };
                    return Err(err);
                }
                let output = TaskOutput::StandardOutput {
                    stdout: str::from_utf8(&py_result.stdout).unwrap().parse().unwrap(),
                    stderr: str::from_utf8(&py_result.stderr).unwrap().parse().unwrap(),
                };
                Ok(output)
            }
            Err(err) => {
                error!("Python script crashed: {}", err);
                let msg_err = format!("Python script error: {:?}", err);
                let err = YoshiError {
                    message: msg_err,
                    origin: "PythonTaskDefinition::run".to_owned(),
                };
                Err(err)
            }
        }
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
    pub fn new(script_path: FilePath, args: Vec<String>) -> Self {
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
