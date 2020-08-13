use crate::type_definition::{DateTimeUtc, FilePath, RunnerId, TaskId};

/// Status of a TaskInstance
#[derive(Clone, PartialEq)]
pub enum TaskStatus {
    Defined,
    Queued,
    Success,
    Failure,
}

// todo: define psql connection
type PostgresConn = String;

/// Possible output to recover from a TaskDefinition
#[derive(Clone)]
pub enum TaskOutput {
    Text(String),
    LocalFile(Box<FilePath>),
    PostgresTable(PostgresConn, String),
}

/// The result of a Task that ran through a Runner
#[derive(Clone)]
pub struct TaskInstance {
    pub id_task_definition: TaskId,
    pub id_task_runner: RunnerId,
    pub date_started: DateTimeUtc,
    pub date_finished: DateTimeUtc,
    pub status: TaskStatus,
    pub got_output: TaskOutput,
}
