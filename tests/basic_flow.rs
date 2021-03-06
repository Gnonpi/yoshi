use chrono::Utc;
use std::fs::{remove_file, File};
use std::io::Write;
use yoshi::dag::Dag;
use yoshi::runners::{LocalTaskRunner, TaskRunner};
use yoshi::task_definition::{DefinitionArguments, TaskDefinitionType};
use yoshi::task_instance::TaskStatus;
use yoshi::task_node::TaskNode;
use yoshi::task_output::TaskOutput;
use yoshi::type_definition::FilePath;
//
mod common;

#[test]
fn can_mount_simple_dag() {
    common::init_logger();

    // Create Python task
    let script_path = FilePath::from("script.py");
    let mut file = File::create(script_path.clone()).unwrap();
    // todo: replace with include_string macro
    file.write_all(include_str!("python_script").as_bytes())
        .unwrap();
    // Create python arguments
    let mut da_py = DefinitionArguments::new();
    da_py.set(&String::from("script_path"), "script.py".to_string());
    da_py.set(&String::from("args"), "[\"ok\"]".to_string());

    // Create bash arguments
    let mut da_ba = DefinitionArguments::new();
    da_ba.set(
        &String::from("command"),
        "[\"cat\", \"/tmp/hello\"]".to_string(),
    );

    // Create a Dag
    let local_id_runner = LocalTaskRunner::new().get_runner_id();
    let python_node = TaskNode::new(TaskDefinitionType::Python, da_py, local_id_runner);
    let bash_node = TaskNode::new(TaskDefinitionType::Bash, da_ba, local_id_runner);
    let python_node_id = python_node.id_node.clone();
    let bash_node_id = bash_node.id_node.clone();
    let mut dag = Dag::new();

    // Adding tasks to dag
    dag.add_task(python_node.clone(), None, None).unwrap();
    dag.add_task(bash_node.clone(), Some(vec![&python_node_id]), None)
        .unwrap();

    // Running
    let date_before_run = Utc::now();
    let res_dag = dag.run();
    let date_after_run = Utc::now();
    assert!(res_dag.is_ok());

    // Access results from DAG
    let complete_python_node = dag.get_node(&python_node_id);
    assert!(complete_python_node.is_some());
    assert!((*complete_python_node.unwrap()).instance.is_some());

    let python_task_instance = (*complete_python_node.unwrap())
        .instance
        .as_ref()
        .unwrap()
        .clone();
    assert_eq!(python_task_instance.id_node, python_node_id);
    // TaskDefinition are now created on the fly
    // assert_eq!(
    //     python_task_instance.id_task_definition,
    //     python_def.task_definition_id()
    // );
    assert_eq!(python_task_instance.id_task_runner, python_node.id_runner);
    assert!(python_task_instance.date_started > date_before_run);
    assert!(python_task_instance.date_started < date_after_run);
    assert_eq!(python_task_instance.status, TaskStatus::Success);
    assert_eq!(
        python_task_instance.output,
        TaskOutput::StandardOutput {
            stdout: String::from(""),
            stderr: String::from("")
        }
    );

    let complete_bash_node = dag.get_node(&bash_node_id);
    assert!(complete_bash_node.is_some());
    assert!((*complete_bash_node.unwrap()).instance.is_some());

    let bash_task_instance = (*complete_bash_node.unwrap())
        .instance
        .as_ref()
        .unwrap()
        .clone();
    assert_eq!(bash_task_instance.id_node, bash_node_id);
    // TaskDefinition are now created on the fly
    assert_eq!(bash_task_instance.id_task_runner, bash_node.id_runner);
    assert!(bash_task_instance.date_started > date_before_run);
    assert!(bash_task_instance.date_started < date_after_run);
    assert!(bash_task_instance.date_started > python_task_instance.date_started);

    assert_eq!(bash_task_instance.status, TaskStatus::Success);
    assert_eq!(
        bash_task_instance.output,
        TaskOutput::StandardOutput {
            stdout: String::from("Hello ok"),
            stderr: String::from("")
        }
    );

    // Cleanup
    remove_file(script_path).unwrap();
    remove_file("/tmp/hello").unwrap();
}
