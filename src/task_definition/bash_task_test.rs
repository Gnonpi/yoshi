use crate::task_definition::bash_task::*;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

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
                let result_str = std::str::from_utf8(&stdout).unwrap();
                assert_eq!(result_str, "'Hello'\n");
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
