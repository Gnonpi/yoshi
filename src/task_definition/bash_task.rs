use crate::errors::YoshiError;
use crate::task_definition::{generate_task_definition_id, TaskDefinition};
use crate::task_output::TaskOutput;
use crate::type_definition::TaskId;
use log::{debug, error, info};
use std::collections::HashMap;
use std::process::Command;
use std::str;

/// A Bash task that runs a Bash command
#[derive(Clone, Debug)]
pub struct BashTaskDefinition {
    task_def_id: TaskId,
    command: Vec<String>,
}

impl TaskDefinition for BashTaskDefinition {
    fn task_definition_id(&self) -> TaskId {
        self.task_def_id
    }
    fn run(&self) -> Result<TaskOutput, YoshiError> {
        info!("Starting Bash command {:?}", self.command);
        let bash_proc = Command::new(self.command[0].clone())
            .args(&self.command[1..self.command.len()])
            .output();
        match bash_proc {
            Ok(bash_result) => {
                debug!("bash stdout: {:?}", bash_result.stdout);
                if !bash_result.status.success() {
                    error!("Bash command crashed");
                    let err = YoshiError {
                        message: "Bash command was not a success".to_owned(),
                        origin: "BashTaskDefinition::run".to_owned(),
                    };
                    return Err(err);
                }
                let output = TaskOutput::StandardOutput {
                    stdout: str::from_utf8(&bash_result.stdout)
                        .unwrap()
                        .parse()
                        .unwrap(),
                    stderr: str::from_utf8(&bash_result.stderr)
                        .unwrap()
                        .parse()
                        .unwrap(),
                };
                Ok(output)
            }
            Err(err) => {
                error!("Bash command crashed: {:?}", err);
                let msg_err = format!("Bash cmd error: {:?}", err);
                let err = YoshiError {
                    message: msg_err,
                    origin: "BashTaskDefinition::run".to_owned(),
                };
                return Err(err);
            }
        }
    }

    fn get_params(&self) -> HashMap<String, String> {
        debug!("Getting bash command parameters");
        let mut params: HashMap<String, String> = HashMap::new();
        let mut total_command = self.command[0].clone();
        for (i, cmd) in self.command.iter().enumerate() {
            if i == 0 {
                continue;
            }
            total_command = total_command + " " + cmd
        }
        params.insert("command".to_owned(), total_command);
        params
    }
}

impl BashTaskDefinition {
    pub fn new(command: Vec<String>) -> Self {
        debug!("Creating Bash task definition");
        BashTaskDefinition {
            task_def_id: generate_task_definition_id(),
            command,
        }
    }
}

#[cfg(test)]
#[path = "./bash_task_test.rs"]
mod bash_task_test;
