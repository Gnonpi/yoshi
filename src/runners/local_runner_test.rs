use crate::runners::{FailureReason, LocalTaskRunner, MessageFromRunner, TaskRunner};
use crate::task_definition::{BashTaskDefinition, TaskDefinition};
use crate::task_instance::TaskStatus;
use crate::type_definition::NodeId;

fn _produce_task_def() -> BashTaskDefinition {
    BashTaskDefinition::new(vec!["echo".to_string(), "'Hola'".to_string()])
}

#[test]
fn it_can_run_a_simple_bash_command() {
    let mut ltr = LocalTaskRunner::new();
    let node_id = NodeId::new_v4();
    let task_def = _produce_task_def();
    let (snd_to, rcv_from) = ltr.get_channels();
    let res = ltr.start_task(node_id.clone(), &task_def);
    assert!(res.is_ok());
    assert!(snd_to.is_empty());
    assert_eq!(rcv_from.len(), 1);
    let msg = rcv_from.recv().unwrap();
    match msg {
        MessageFromRunner::Done {
            start_time,
            end_time,
        } => {
            assert!(start_time < end_time);
            assert_eq!(ltr.get_status(), TaskStatus::Success);
        }
        _ => {
            panic!("Expected DONE message from Runner");
        }
    }
}

#[test]
fn it_dont_crash_on_task_error() {
    let task_def = BashTaskDefinition::new(vec!["thiscommanddontexist".to_string()]);
    let mut ltr = LocalTaskRunner::new();
    let node_id = NodeId::new_v4();
    let (snd_to, rcv_from) = ltr.get_channels();
    let res = ltr.start_task(node_id.clone(), &task_def);
    assert!(res.is_ok());
    assert!(snd_to.is_empty());
    assert_eq!(rcv_from.len(), 1);
    // Err here would mean a problem communicating with runner,
    // which shouldn't happen
    let msg = rcv_from.recv().unwrap();
    match msg {
        MessageFromRunner::Failure {
            start_time,
            failure_time,
            reason,
        } => {
            assert!(start_time < failure_time);
            match reason {
                FailureReason::GotError(_) => {
                    println!("ok");
                }
                _ => {
                    panic!("FailureReason should be a String");
                }
            }
        }
        _ => {
            panic!("Task failing should mean FAILURE message");
        }
    }
}

#[test]
fn it_returns_the_task_instance() {
    let mut ltr = LocalTaskRunner::new();
    let node_id = NodeId::new_v4();
    let task_def = _produce_task_def();
    let (snd_to, rcv_from) = ltr.get_channels();
    let res = ltr.start_task(node_id.clone(), &task_def);
    assert!(res.is_ok());
    let s_instance = ltr.get_task_instance();
    assert!(s_instance.is_some());
    let instance = s_instance.unwrap();
    assert_eq!(instance.id_node, node_id);
    assert_eq!(instance.id_task_definition, task_def.task_definition_id());
    assert_eq!(instance.id_task_runner, ltr.get_runner_id());
    assert_eq!(instance.status, TaskStatus::Success);
    // assert!(instance.output.is_some());
}

#[test]
fn it_crash_when_starting_before_getting_the_channels() {
    let mut ltr = LocalTaskRunner::new();
    let node_id = NodeId::new_v4();
    let task_def = _produce_task_def();
    let res = ltr.start_task(node_id, &task_def);
    assert!(res.is_err());
}
