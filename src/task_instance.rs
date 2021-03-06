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
    // todo: TaskId doesn't allow to recall what's been done if we don't store what it's doing
    pub id_task_definition: TaskId,
    pub id_task_runner: RunnerId,
    pub date_started: DateTimeUtc,
    pub date_finished: DateTimeUtc,
    pub status: TaskStatus,
    pub output: TaskOutput,
}
