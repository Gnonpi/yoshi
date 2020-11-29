use crate::type_definition::FilePath;

// todo: define psql connection
type PostgresConn = String;

/// Possible output to recover from a TaskDefinition
#[derive(Clone, PartialEq, Debug)]
pub enum TaskOutput {
    Nothing,
    Text(String),
    LocalFile(Box<FilePath>),
    StandardOutput { stdout: String, stderr: String },
    PostgresTable(PostgresConn, String),
}
