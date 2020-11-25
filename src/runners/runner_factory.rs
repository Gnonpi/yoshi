use super::{FakeTaskRunner, LocalTaskRunner, TaskRunner};

/// Enum to selec the type of runner to create
/// TaskRunner impl a method that returns one of those
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TaskRunnerType {
    Fake,
    LocalBlocking,
}

pub fn string_to_runner_type(runner_name: String) -> Option<TaskRunnerType> {
    match runner_name.as_str() {
        "local_runner" => return Some(TaskRunnerType::LocalBlocking),
        _ => return None,
    }
}

pub struct TaskRunnerFactory;

// todo: should we use factory for TaskDefinition?
// how to pass initialization parameters to Runner? adding Builder pattern?
impl TaskRunnerFactory {
    pub fn new_runner(trt: &TaskRunnerType) -> Box<dyn TaskRunner> {
        match trt {
            TaskRunnerType::Fake => Box::new(FakeTaskRunner {}),
            TaskRunnerType::LocalBlocking => Box::new(LocalTaskRunner::new()),
        }
    }
}

#[cfg(test)]
#[path = "./runner_factory_test.rs"]
mod runner_factory_test;
