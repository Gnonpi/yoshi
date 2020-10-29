use crate::task_definition::python_task::*;
use crate::type_definition::FilePath;
use std::boxed::Box;
use std::fs::{remove_file, File};
use std::io::prelude::*;

// todo: move this to test utils module and reuse?
fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn it_can_run_basic_script() {
    init_logger();

    let script_path = FilePath::from("script.py");
    let mut file = File::create(script_path.clone()).unwrap();
    file.write_all(b"import sys; a = sys.argv[1]; print('all good {}'.format(a))")
        .unwrap();
    let args = vec!["one".to_string()];
    let ptd = PythonTaskDefinition {
        task_def_id: generate_task_definition_id(),
        script_path: Box::new(script_path.clone()),
        args,
    };
    let res = ptd.run();
    assert!(res.is_ok());

    if let Ok(py_output) = res {
        match py_output {
            TaskOutput::StandardOutput { stdout, stderr } => {
                assert_eq!(stdout, String::from("all good one\n"));
            }
            _ => {
                panic!("py_output should be a TaskOutput::StandardOuput");
            }
        }
    }

    remove_file(script_path).unwrap();
}

#[test]
fn it_can_return_parameters() {
    let script_path = FilePath::from("script.py");
    let args = vec!["one".to_string(), "two".to_string()];
    let ptd = PythonTaskDefinition {
        task_def_id: generate_task_definition_id(),
        script_path: Box::new(script_path.clone()),
        args,
    };
    let params = ptd.get_params();
    let p = script_path.clone().into_string().unwrap();
    assert_eq!(params.get("script_path"), Some(&p));
    assert_eq!(params.get("args"), Some(&"one two ".to_string()));
}
