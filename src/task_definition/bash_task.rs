use super::task_def::{generate_task_definition_id, TaskDefinition};
use crate::errors::YoshiError;
use crate::type_definition::TaskId;
use std::collections::HashMap;
use std::process::Command;
use log::{info, debug, error};

/// A Bash task that runs a Bash command
#[derive(Clone)]
struct BashTaskDefinition {
    task_def_id: TaskId,
    command: Vec<String>,
}

impl TaskDefinition for BashTaskDefinition {
    fn task_definition_id(&self) -> TaskId {
        self.task_def_id
    }
    fn run(&self) -> Result<(), YoshiError> {
        info!("Starting Bash command {:?}", self.command);
        let bash_command = Command::new(self.command[0].clone())
            .args(&self.command[1..self.command.len()])
            .spawn()
            .expect("bash command failed to start");
        let bash_result = bash_command
            .wait_with_output()
            .expect("failed to wait on Bash command");
        debug!("bash stdout: {:?}", bash_result.stdout);
        if !bash_result.status.success() {
            error!("Bash command crashed");
            let err = YoshiError {
                message: "Bash command was not a success".to_owned(),
                origin: "BashTaskDefinition::run".to_owned()
            };
            return Err(err)
        }
        Ok(())
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
    fn new(command: Vec<String>) -> Self {
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