use crate::test_utils::init_logger;
use crate::task_output::TaskOutput;
use crate::task_definition::DefinitionArguments;
use super::SqliteConnector;
use super::parse_sqlite_dsn;
use std::convert::TryFrom;

use crate::task_definition::task_def::TaskDefinition;
use crate::task_definition::db::db_task::DbConnector;

#[test]
fn check_via_raw() {
    init_logger();
    let s_memory = ":memory:".to_string();
    let s = SqliteConnector {
        db_conn_args: parse_sqlite_dsn(&s_memory.clone()),
        dsn: s_memory.clone(),
        query: "SELECT 1 AS number;".to_string()
    };
    let is_conn_ok = s.check_connection();
    assert!(is_conn_ok);
}

#[test]
fn check_via_def_args() {
    init_logger();
    let mut da = DefinitionArguments::new();
    // we want to be able to store everything
    // let query = "SELECT 1 AS number, 'a' AS letter, 1.0 AS my_float;".to_string();
    let query = "SELECT 1 AS number;".to_string();
    da.set(&"dsn".to_string(), ":memory:".to_string());
    da.set(&"query".to_string(), query);
    let s = SqliteConnector::try_from(da).unwrap();
    let result = s.run().unwrap();

    match result {
        TaskOutput::SqlQueryResult { 
            rows: result_rows
        } => {
            println!("{:?}", result_rows);
            assert_eq!(2, 3);
        },
        _ => {
            panic!("Expected SqlQueryResult, got {:?}", result);
        }
    }

    assert_eq!(1, 2);
}
