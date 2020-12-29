use crate::test_utils::init_logger;
// use crate::task_output::TaskOutput;
use super::*;

#[test]
fn it_can_run_small_function() {
    fn say_hello() {
        println!("Hello");
    }

    init_logger();
    let rftd = RustFunctionTaskDefinition {
        task_def_id: generate_task_definition_id(),
        callable: say_hello
    };
    let res = rftd.run().unwrap();
    assert_eq!(res, TaskOutput::Nothing);
}

#[test]
fn it_stops_on_panic() {
    fn it_panics() {
        panic!("This function will panic!");
    }
    init_logger();
    let rftd = RustFunctionTaskDefinition {
        task_def_id: generate_task_definition_id(),
        callable: it_panics
    };
    let res = rftd.run();
    assert!(res.is_err());
}
