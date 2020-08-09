use crate::type_definition::{DateTimeUtc, FilePath, RunnerId, TaskId};

/// Status of a TaskInstance
#[derive(PartialEq)]
pub enum TaskStatus {
    Defined,
    Queued,
    Success,
    Failure,
}

#[derive(Clone)]
pub enum TaskOutput {
    ArtifactPath(Box<FilePath>),
    Text(String),
}

/// The result of a Task that ran through a Runner
pub struct TaskInstance {
    pub id_task_definition: TaskId,
    pub id_task_runner: RunnerId,
    pub date_started: DateTimeUtc,
    pub date_finished: DateTimeUtc,
    pub status: TaskStatus,
    pub got_output: TaskOutput,
}
