use crate::task_definition::*;
use crate::type_definition::FilePath;
use std::fs::File;
use std::io::prelude::*;
use std::boxed::Box;

#[test]
fn it_can_run_basic_script() {
    let script_path = FilePath::from("script.py");
    let mut file = File::create(script_path.clone()).unwrap();
    file.write_all(b"import sys; a = sys.argv[1]; print(f'all good {a}')").unwrap();
    let args = vec!["one".to_string()];
    let ptd = PythonTaskDefinition {
        task_def_id: 0,
        script_path: Box::new(script_path.clone()),
        args,
    };
    let res = ptd.run();
    assert!(res.is_ok());
    assert!(false);
}

#[test]
fn it_can_return_parameters() {
    let script_path = FilePath::from("script.py");
    let args = vec!["one".to_string(), "two".to_string()];
    let ptd = PythonTaskDefinition {
        task_def_id: 0,
        script_path: Box::new(script_path.clone()),
        args,
    };
    let params = ptd.get_params();
    let p = script_path.clone().into_string().unwrap();
    assert_eq!(params.get("script_path"), Some(&p));
    assert_eq!(params.get("args"), Some(&"one two ".to_string()));
}
