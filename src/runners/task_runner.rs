use crate::task_definition::TaskDefinition;
use crate::task_instance::{TaskInstance, TaskStatus};
use crate::type_definition::{DateTimeUtc, NodeId, RunnerId};
use crossbeam_channel::{Receiver, Sender};
use dyn_clone::DynClone;
use std::fmt::Debug;

pub enum FailureReason {
    GotError(String),
    Cancelled(DateTimeUtc),
}

// todo: are Message From/To XXX the best?
pub enum MessageFromRunner {
    Queued,
    Running {
        start_time: DateTimeUtc,
    },
    Done {
        start_time: DateTimeUtc,
        end_time: DateTimeUtc,
    },
    Paused {
        start_time: DateTimeUtc,
        pause_time: DateTimeUtc,
    },
    Failure {
        start_time: DateTimeUtc,
        reason: FailureReason,
        failure_time: DateTimeUtc,
    },
}

pub enum MessageToRunner {
    GetStatus,
    Pause,
    Cancel,
}

/// Struct in charge of taking a TaskDefinition 
/// and run it somewhere 
/// and create the TaskInstance when it finishes
pub trait TaskRunner: DynClone + Debug {
    /// Get an identifier of the Runner
    fn get_runner_id(&self) -> RunnerId;
    /// Start the task and gives 2 channels to communicate while it's running
    fn start_task(
        &self,
        node_id: NodeId,
        task_def: &dyn TaskDefinition,
    ) -> (Sender<MessageToRunner>, Receiver<MessageFromRunner>);
    /// Get the status while it's running
    fn get_status(&self) -> TaskStatus;
    /// Get the resulting TaskInstance if it's done
    fn get_task_instance(&self) -> Option<TaskInstance>;
}

dyn_clone::clone_trait_object!(TaskRunner);
