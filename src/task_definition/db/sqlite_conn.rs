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
use log::{debug, info};

use rusqlite::{params, Connection, MappedRows};
use rusqlite::Result as RQLiteResult;
use serde_rusqlite;
use serde_json;

/// Connector to a SQLite database
#[derive(Debug)]
pub struct SqliteConnector {
    dsn: String,
    db_conn_args: DbConnectionArguments,
    query: String,
}

/*
This is going to be dirty
*/
fn map_values_to_json() -> {
    
}

fn row_to_hashmap(row: &rusqlite::Row<'_>) -> Result<HashMap<String, rusqlite::types::Value>, rusqlite::Error> {
    let map = HashMap::new();
    let col_count = row.column_count();
    for c in 0..col_count {
        // we take the ValueRef
        let raw_value = row.get_raw(c);
        // turn ValueRef into Value
        let value = rusqlite::types::Value::from(raw_value);
        let col_name = row.column_name(c).unwrap();
        map.insert(col_name.to_string(), value);
    }
    Ok(map)
}

impl DbConnector for SqliteConnector {
    fn get_dsn(&self) -> DbConnectionArguments {
        self.db_conn_args.clone()
    }

    fn run_query(
        &self,
        query: &String,
        parameters: Option<&QueryParameters>,
    ) -> Result<QueryResult, YoshiError>{
        let conn;
        match &self.db_conn_args {
            DbConnectionArguments::Memory => {
                conn = Connection::open_in_memory().unwrap();
            },
            DbConnectionArguments::File(fp) => {
                conn = Connection::open(fp).unwrap()
            },
            _ => {
                // todo: add variant in YoshiError, is that possible
                panic!("Can only connect to SQLite via memory or filepath, got {:?}", self.db_conn_args);
            }
        }

        // https://stackoverflow.com/questions/60396593/how-do-i-use-rusqlites-rowget-method-for-a-type-which-i-dont-know-at-compile
        debug!("Preparing to run query: {:?}", query);
        let mut stmt = conn.prepare(query).unwrap();
        debug!("Executing statement");
        /*
        let mut rows = stmt.query_map(params![], |row| Ok( 
            row.get(0)
        )).unwrap();
        */
        // let rows = serde_rusqlite::from_rows::<BothWaySerde>(stmt.query(params![]).unwrap());
        let rows = stmt.query_map(params![], |row| row_to_hashmap(row));
        let mut result = Vec::new();
        for r in rows {
            let value = r;
            let json_value = serde_json::to_string(&value).unwrap();
            result.push(json_value);
        }

        Ok(QueryResult {
            nb_rows: result.len(),
            rows: result
        })
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

#[cfg(test)]
#[path = "./sqlite_conn_test.rs"]
mod sqlite_conn_test;

