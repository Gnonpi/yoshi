use crate::type_definition::{DateTimeUtc, NodeId, RunnerId};
use crate::task_instance::{TaskInstance, TaskStatus};
use crate::task_definition::TaskDefinition;
use crossbeam_channel::{Sender, Receiver};

enum FailureReason {
    GotError(String),
    Cancelled(DateTimeUtc)
}

// todo: are Message From/To XXX the best?
enum MessageFromRunner {
    Queued,
    Running { 
        start_time: DateTimeUtc 
    },
    Done { 
        start_time: DateTimeUtc,
        end_time: DateTimeUtc
    },
    Paused {
        start_time: DateTimeUtc,
        pause_time: DateTimeUtc
    },
    Failure {
        start_time: DateTimeUtc,
        reason: FailureReason,
        failure_time: DateTimeUtc
    }
}

enum MessageToRunner {
    GetStatus,
    Pause,
    Cancel
}

trait TaskRunner {
    fn get_runner_id(&self) -> RunnerId;
    fn start_task(&self, node_id: NodeId, task_def: dyn TaskDefinition) -> (Sender<MessageToRunner>, Receiver<MessageFromRunner>);
    fn get_status(&self) -> TaskStatus;
    fn get_task_instance(&self) -> Option<TaskInstance>;
}
