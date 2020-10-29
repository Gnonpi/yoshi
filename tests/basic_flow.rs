use yoshi::task_node::TaskNode;
use yoshi::task_definition::{PythonTaskDefinition, BashTaskDefinition};
use std::fs::{File, remove_file};
use yoshi::type_definition::FilePath;
use std::io::Write;
use yoshi::runners::{TaskRunnerFactory, LocalTaskRunner, TaskRunner};
use yoshi::dag::Dag;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn can_mount_simple_dag() {
    init_logger();

    // Create Python task
    let script_path = FilePath::from("script.py");
    let mut file = File::create(script_path.clone()).unwrap();
    file.write_all(
    b"import sys \
a = sys.argv[1] \
with open('tmp/hello', 'w') as f:\
    f.write('Hello {}'.format(a))
    "
    )
        .unwrap();
    let python_def = PythonTaskDefinition::new(
        script_path.clone(),
        vec!["ok".to_owned()]
    );

    // Create bash task
    let bash_command = vec!["echo".to_owned(), "/tmp/hello".to_owned()];
    let bash_def = BashTaskDefinition::new(bash_command);

    // Create a Dag
    let local_id_runner = LocalTaskRunner::new().get_runner_id();
    let python_node = TaskNode::new(Box::new(python_def), local_id_runner);
    let bash_node = TaskNode::new(Box::new(bash_def), local_id_runner);
    let python_node_id = python_node.id_node.clone();
    let mut dag = Dag::new();
    dag.add_task(python_node, None, None);
    dag.add_task(bash_node, Some(vec![&python_node_id]), None);
    dag.set_starting_node(python_node_id);
    let res_dag = dag.run();
    assert!(res_dag.is_ok());

    // Cleanup
    remove_file(script_path).unwrap();
}
