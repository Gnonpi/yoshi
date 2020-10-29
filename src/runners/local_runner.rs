use crate::runners::task_runner::ChannelsNotAcquiredBeforeStartingError;
use crate::runners::{
    FailureReason, MessageFromRunner, MessageToRunner, TaskRunner, TaskRunnerType,
};
use crate::task_definition::TaskDefinition;
use crate::task_instance::{TaskInstance, TaskStatus};
use crate::type_definition::{NodeId, RunnerId};
use chrono::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use log::{debug, warn};

#[derive(Debug, Clone)]
pub struct LocalTaskRunner {
    current_status: TaskStatus,
    stored_instance: Option<TaskInstance>,
    channels_acquired: bool,
    recv_to_runner: Option<Receiver<MessageToRunner>>,
    send_from_runner: Option<Sender<MessageFromRunner>>,
}

impl TaskRunner for LocalTaskRunner {
    fn get_runner_id(&self) -> RunnerId {
        TaskRunnerType::LocalBlocking
    }

    fn get_channels(&mut self) -> (Sender<MessageToRunner>, Receiver<MessageFromRunner>) {
        let (send_to_runner, recv_to_runner) = unbounded::<MessageToRunner>();
        let (send_from_runner, recv_from_runner) = unbounded::<MessageFromRunner>();

        self.channels_acquired = true;
        self.recv_to_runner = Some(recv_to_runner);
        self.send_from_runner = Some(send_from_runner);

        (send_to_runner, recv_from_runner)
    }

    /// Start a task, blocking the thread
    fn start_task(
        &mut self,
        node_id: NodeId,
        task_def: &dyn TaskDefinition,
    ) -> Result<(), ChannelsNotAcquiredBeforeStartingError> {
        if !self.channels_acquired {
            return Err(ChannelsNotAcquiredBeforeStartingError {});
        }

        debug!("Start running task in Local runner");
        let start_time = Utc::now();
        let task_result = task_def.run();
        let end_time = Utc::now();
        match task_result {
            Ok(output) => {
                debug!("Task done, creating corresponding TaskInstance");
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
                    output
                };
                self.current_status = TaskStatus::Success;
                self.stored_instance = Some(inst);
                debug!("Sending SUCCESS message");
                self.send_from_runner
                    .as_ref()
                    .unwrap()
                    .send(msg_success)
                    .unwrap();
            }
            Err(err) => {
                warn!("Task failed {:?} {:?}", task_def, self);
                let err_msg = format!("{:?}", err);
                let msg_failure = MessageFromRunner::Failure {
                    start_time,
                    reason: FailureReason::GotError(err_msg),
                    failure_time: end_time,
                };
                self.current_status = TaskStatus::Failure;
                debug!("Sending FAILURE message");
                self.send_from_runner
                    .as_ref()
                    .unwrap()
                    .send(msg_failure)
                    .unwrap();
            }
        }
        Ok(())
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
        debug!("Creating new LocalTaskRunner");
        LocalTaskRunner {
            current_status: TaskStatus::Undefined,
            stored_instance: None,
            channels_acquired: false,
            recv_to_runner: None,
            send_from_runner: None,
        }
    }
}

impl Default for LocalTaskRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "./local_runner_test.rs"]
mod dag_test;
