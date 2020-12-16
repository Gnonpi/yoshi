use std::collections::HashMap;
use crate::errors::YoshiError;
use crate::task_output::TaskOutput;
use crate::task_definition::TaskDefinitionType;
use crate::type_definition::TaskId;
use crate::task_definition::{TaskDefinition, generate_task_definition_id};
use log::{debug, info};

pub struct DbConnectionArguments {}
pub struct DbConnection {}

pub struct QueryParameters {}
pub struct QueryResult {}

pub trait DbConnector {
    fn create_connection(db_args: DbConnectionArguments) -> Self;
    fn check_connection(&self) -> bool {
        let select_one = "SELECT 1;";
        let result = self.query(select_one, None);
        result.is_ok()
    }
    fn query(query: String, parameters: Option<QueryParameters>) -> Result<QueryResult, YoshiError>;
}

struct PostgresConnector {}
struct SqliteConnector {}

/*
impl TaskDefinition for PostgresConnector {
    fn task_definition_id(&self) -> TaskId;
    fn task_type(&self) -> TaskDefinitionType;
    fn run(&self) -> Result<TaskOutput, YoshiError>;
    fn get_params(&self) -> HashMap<String, String>;
}
*/

impl TaskDefinition for SqliteConnector {
    fn task_definition_id(&self) -> TaskId {
        generate_task_definition_id()
    }
    fn task_type(&self) -> TaskDefinitionType {
        TaskDefinitionType::Sqlite
    }
    fn run(&self) -> Result<TaskOutput, YoshiError> {

    }
    fn get_params(&self) -> HashMap<String, String>;
}

