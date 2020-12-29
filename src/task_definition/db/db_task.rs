use crate::errors::YoshiError;
use crate::task_output::TaskOutput;
use crate::type_definition::FilePath;
use std::convert::TryFrom;

/// URI to connect to a database
#[derive(Debug, Clone)]
pub struct DbConnectionArgUri {
    host: String,
    port: String,
    user: String,
    password: String,
    dbname: String,
}

/// The point to connect to a DB
#[derive(Debug, Clone)]
pub enum DbConnectionArguments {
    Memory,
    File(FilePath),
    Uri(DbConnectionArgUri),
}

/// Parameters to pass to run the query
pub struct QueryParameters {}

/// The rows returned by the query
// left empty for now as it's complicated to run a query 
// and store the results without knowing the structs to store them in
pub struct QueryResult {
    pub nb_rows: usize,
    // pub rows: Vec<String>   // json string
}

/// How to connect and interact with a database
pub trait DbConnector {
    /// Returns a description of the connection
    fn get_dsn(&self) -> DbConnectionArguments;

    /// Run a query to check that the DB can be used
    fn check_connection(&self) -> bool;

    /// Given a query string and possibly parameters, and returns its results
    fn run_query(
        &self,
        query: &String,
        parameters: Option<&QueryParameters>,
    ) -> Result<QueryResult, YoshiError>;
}

/// Convert the rows from a query to a TaskOutput
impl TryFrom<QueryResult> for TaskOutput {
    type Error = YoshiError;

    fn try_from(qr: QueryResult) -> Result<TaskOutput, Self::Error> {
        Ok(TaskOutput::SqlQueryResult { 
            nb_rows: qr.nb_rows
        })
    }
}

struct PostgresConnector {}
