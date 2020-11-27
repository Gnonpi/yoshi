use crate::task_definition::bash_task::*;
use crate::test_utils::init_logger;

#[test]
fn it_can_run_basic_command() {
    init_logger();
    let vec_cmd = vec!["echo".to_owned(), "'Hello'".to_owned()];
    let btd = BashTaskDefinition {
        task_def_id: generate_task_definition_id(),
        command: vec_cmd,
    };
    let result = btd.run();
    assert!(result.is_ok());

    if let Ok(bash_output) = result {
        match bash_output {
            TaskOutput::StandardOutput { stdout, stderr } => {
                assert_eq!(stdout, String::from("'Hello'\n"));
            }
            _ => {
                panic!("bash_output should be a TaskOutput::StandardOuput");
            }
        }
    }
}

#[test]
fn it_can_return_parameters() {
    let vec_cmd = vec!["echo".to_owned(), "'Hello'".to_owned()];
    let btd = BashTaskDefinition {
        task_def_id: generate_task_definition_id(),
        command: vec_cmd,
    };
    let result = btd.get_params();
    assert_eq!(result.len(), 1);
    assert_eq!(
        result.get("command").unwrap().to_owned(),
        "echo \'Hello\'".to_owned()
    );
}
