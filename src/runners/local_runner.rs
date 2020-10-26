use crate::errors::YoshiError;
use crate::runners::{FailureReason, MessageFromRunner, MessageToRunner, TaskRunner, TaskRunnerType};
use crate::task_definition::TaskDefinition;
use crate::task_instance::{TaskInstance, TaskStatus};
use crate::task_output::TaskOutput;
use crate::type_definition::{DateTimeUtc, NodeId, RunnerId};
use chrono::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use log::warn;

#[derive(Debug, Clone)]
pub struct LocalTaskRunner {
    current_status: TaskStatus,
    stored_instance: Option<TaskInstance>,
}

impl TaskRunner for LocalTaskRunner {
    fn get_runner_id(&self) -> RunnerId {
        return TaskRunnerType::LocalBlocking;
    }

    /// Start a task, blocking the thread
    fn start_task(
        &mut self,
        node_id: NodeId,
        task_def: &dyn TaskDefinition,
    ) -> (Sender<MessageToRunner>, Receiver<MessageFromRunner>) {
        let (send_to_runner, recv_to_runner) = unbounded::<MessageToRunner>();
        let (send_from_runner, recv_from_runner) = unbounded::<MessageFromRunner>();

        let start_time = Utc::now();
        let task_result = task_def.run();
        let end_time = Utc::now();
        match task_result {
            Ok(output) => {
                let msg_success = MessageFromRunner::Done {
                    start_time,
                    end_time,
                };
                let inst = TaskInstance {
                    id_node: node_id,
                    id_task_definition: task_def.task_definition_id(),
                    id_task_runner: self.get_runner_id(),
                    date_started: start_time,
                    date_finished: end_time,
                    status: TaskStatus::Success,
                    output: output,
                };
                self.current_status = TaskStatus::Success;
                self.stored_instance = Some(inst);
                send_from_runner.send(msg_success);
            }
            Err(err) => {
                warn!("Task failed {:?} {:?}", task_def, self);
                let err_msg = format!("{:?}", err);
                let msg_failure = MessageFromRunner::Failure {
                    start_time: start_time,
                    reason: FailureReason::GotError(err_msg),
                    failure_time: end_time,
                };
                self.current_status = TaskStatus::Failure;
                send_from_runner.send(msg_failure);
            }
        }

        (send_to_runner, recv_from_runner)
    }

    fn get_status(&self) -> TaskStatus {
        self.current_status.clone()
    }

    fn get_task_instance(&self) -> Option<TaskInstance> {
        self.stored_instance.clone()
    }
}

impl LocalTaskRunner {
    pub fn new() -> Self {
        LocalTaskRunner {
            current_status: TaskStatus::Undefined,
            stored_instance: None,
        }
    }
}

#[cfg(test)]
#[path = "./local_runner_test.rs"]
mod dag_test;
