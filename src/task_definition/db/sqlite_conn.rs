use crate::errors::YoshiError;
use crate::task_definition::db::{
    DbConnectionArguments, 
    DbConnector, 
    QueryParameters, 
    QueryResult,
};
use crate::task_definition::{
    generate_task_definition_id, 
    DefinitionArgumentElement, 
    DefinitionArgumentType,
    DefinitionArguments, 
    TaskDefinition, 
    TaskDefinitionType,
};
use crate::task_output::TaskOutput;
use crate::type_definition::FilePath;
use crate::type_definition::TaskId;
use std::collections::HashMap;
use std::convert::TryFrom;
use log::{debug, error, info};

use rusqlite::{Connection, NO_PARAMS};

/// Connector to a SQLite database
#[derive(Debug)]
pub struct SqliteConnector {
    dsn: String,
    db_conn_args: DbConnectionArguments,
    query: String,
}

impl DbConnector for SqliteConnector {
    fn get_dsn(&self) -> DbConnectionArguments {
        self.db_conn_args.clone()
    }

    fn check_connection(&self) -> bool {
        let conn = match self.get_connection() {
            Ok(c) => c,
            Err(err) => {
                return false
            }
        };
        let select_one = "SELECT 1 AS number;";
        match conn.query_row(&self.query, NO_PARAMS, |row| row.get::<usize, u32>(0)) {
            Ok(value) => {
                if value == 1 {
                    return true
                }
                return false
            },
            Err(err) => {
                error!("Could not reach DB: {}", err);
                return false
            }
        }
    }

    fn run_query(
        &self,
        query: &String,
        parameters: Option<&QueryParameters>,
    ) -> Result<QueryResult, YoshiError>{
        let conn = self.get_connection().unwrap();
        info!("Executing query");
        match conn.execute(query, NO_PARAMS) {
            Ok(updated) => {
                debug!("Query modified {:?} rows", updated);
                return Ok(QueryResult {
                    nb_rows: updated,
                })
            },
            Err(err) => {
                error!("Error while running query: {}", err);
                let err_msg = format!("{}", err);
                return Err(YoshiError::TaskDefinitionRunFailure(err_msg))
            }
        }
    }
}

// todo: make faillable
pub(crate) fn parse_sqlite_dsn(dsn: &String) -> DbConnectionArguments {
    if dsn == &String::from(":memory:") {
        DbConnectionArguments::Memory
    } else if dsn.ends_with(".db") {
        DbConnectionArguments::File(FilePath::from(dsn))
    } else {
        // todo: add error here
        DbConnectionArguments::Memory
    }
}

impl TryFrom<DefinitionArguments> for SqliteConnector {
    type Error = YoshiError;

    fn try_from(da: DefinitionArguments) -> Result<SqliteConnector, YoshiError> {
        let mut dsn = String::from("");
        let mut query = String::from("");
        if let Some(e) = da.get(&"dsn".to_string(), DefinitionArgumentType::AString) {
            match e {
                DefinitionArgumentElement::AString(s) => {
                    dsn = s;
                }
                _ => {
                    return Err(YoshiError::WrongTypeDefinitionArgumentEntry(
                        "dsn".to_string(),
                        DefinitionArgumentType::AString,
                    ))
                }
            }
        } else {
            return Err(YoshiError::MissingDefinitionArgumentEntry(
                "dsn".to_string(),
            ));
        }

        let db_conn_args = parse_sqlite_dsn(&dsn);

        if let Some(e) = da.get(&"query".to_string(), DefinitionArgumentType::AString) {
            match e {
                DefinitionArgumentElement::AString(s) => {
                    query = s;
                }
                _ => {
                    return Err(YoshiError::WrongTypeDefinitionArgumentEntry(
                        "query".to_string(),
                        DefinitionArgumentType::AString,
                    ))
                }
            }
        } else {
            return Err(YoshiError::MissingDefinitionArgumentEntry(
                "query".to_string(),
            ));
        }
        let connector = SqliteConnector {
            dsn,
            db_conn_args,
            query,
        };
        Ok(connector)
    }
}

impl TaskDefinition for SqliteConnector {
    fn task_definition_id(&self) -> TaskId {
        generate_task_definition_id()
    }

    fn task_type(&self) -> TaskDefinitionType {
        TaskDefinitionType::Sqlite
    }

    fn run(&self) -> Result<TaskOutput, YoshiError> {
        let query_result = self.run_query(&self.query, None).unwrap();
        TaskOutput::try_from(query_result)
    }

    fn get_params(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

impl SqliteConnector {
    fn get_connection(&self) -> Result<Connection, ()> {
        let conn = match &self.db_conn_args {
            DbConnectionArguments::Memory => {
                Connection::open_in_memory().unwrap()
            },
            DbConnectionArguments::File(fp) => {
                Connection::open(fp).unwrap()
            },
            _ => {
                // todo: add variant in YoshiError, is that possible
                panic!("Can only connect to SQLite via memory or filepath, got {:?}", self.db_conn_args);
            }
        };
        Ok(conn)
    }
}

#[cfg(test)]
#[path = "./sqlite_conn_test.rs"]
mod sqlite_conn_test;

