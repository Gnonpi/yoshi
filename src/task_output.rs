use crate::type_definition::FilePath;

/// Possible output to recover from a TaskDefinition
#[derive(Clone, PartialEq, Debug)]
pub enum TaskOutput {
    Nothing,
    Text(String),
    LocalFile(Box<FilePath>),
    StandardOutput { stdout: String, stderr: String },
    SqlQueryResult { rows: Vec<String> }   // stored as json
}
