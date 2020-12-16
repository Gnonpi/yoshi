use crate::task_definition::TaskDefinition;
use crate::task_instance::{TaskInstance, TaskStatus};
use crate::type_definition::{DateTimeUtc, NodeId, RunnerId};
use crossbeam_channel::{Receiver, Sender};
use std::fmt::Debug;

/// Describe the cause of a task stop
#[derive(Debug)]
pub enum FailureReason {
    GotError(String),
    Cancelled(DateTimeUtc),
}

/// The messages that are automatically sent from the TaskRunner to the Dag
#[derive(Debug)]
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

/// Messages that can be sent to a TaskRunner to trigger things
#[derive(Debug)]
pub enum MessageToRunner {
    GetStatus,
    Pause,
    Cancel,
}

#[derive(Debug)]
pub struct ChannelsNotAcquiredBeforeStartingError {}

/// Struct in charge of taking a TaskDefinition
/// and run it somewhere
/// and create the TaskInstance when it finishes
pub trait TaskRunner: Debug {
    /// Get an identifier of the Runner
    fn get_runner_id(&self) -> RunnerId;

    /// Obtain the channels to communicate with TaskRunner when it's working
    /// Channels must have lifetimes bounded by the lifetime of the Runner
    fn get_channels(&mut self) -> (Sender<MessageToRunner>, Receiver<MessageFromRunner>);

    /// Start the task and gives 2 channels to communicate while it's running
    fn start_task(
        &mut self,
        node_id: NodeId,
        task_def: &dyn TaskDefinition,
    ) -> Result<(), ChannelsNotAcquiredBeforeStartingError>;

    /// Get the status while it's running
    fn get_status(&self) -> TaskStatus;

    /// Get the resulting TaskInstance if it's done
    fn get_task_instance(&self) -> Option<TaskInstance>;
}
