use crate::task_definition::*;
use crate::type_definition::FilePath;
use std::boxed::Box;
use std::path::PathBuf;

#[test]
fn it_can_return_parameters() {
    let script_path = FilePath::from("script.py");
    let args = vec!["one".to_string(), "two".to_string()];
    let ptd = PythonTaskDefinition {
        script_path: Box::new(script_path.clone()),
        args,
    };
    let params = ptd.get_params();
    let p = script_path.clone().into_string().unwrap();
    assert_eq!(params.get("script_path"), Some(&p));
    assert_eq!(params.get("args"), Some(&"one two ".to_string()));
}
