use crate::task_output::TaskOutput;
use crate::type_definition::{DateTimeUtc, NodeId, RunnerId, TaskId};

/// Status of a TaskInstance
#[derive(Clone, PartialEq, Debug)]
pub enum TaskStatus {
    Undefined,
    Defined,
    Queued,
    Success,
    Failure,
    Cancelled,
}

/// The result of a Task that ran through a Runner
#[derive(Clone, PartialEq, Debug)]
pub struct TaskInstance {
    pub id_node: NodeId,
    // todo: we have to link id to actual data
    pub id_task_definition: TaskId,
    pub id_task_runner: RunnerId,
    pub date_started: DateTimeUtc,
    pub date_finished: DateTimeUtc,
    pub status: TaskStatus,
    pub output: TaskOutput,
}
