use crate::type_definition::FilePath;


// todo: define psql connection
type PostgresConn = String;

/// Possible output to recover from a TaskDefinition
#[derive(Clone, PartialEq, Debug)]
pub enum TaskOutput {
    Text(String),
    LocalFile(Box<FilePath>),
    StandardOutput {
        stdout: Vec<u8>,
        stderr: Vec<u8>
    },
    PostgresTable(PostgresConn, String),
}
