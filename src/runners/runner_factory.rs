use super::{TaskRunner, FakeTaskRunner, LocalTaskRunner};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TaskRunnerType {
    Fake,
    LocalBlocking
}

pub struct TaskRunnerFactory;

// todo: should we use factory for TaskDefinition?
// how to pass initialization parameters to Runner? adding Builder pattern?
impl TaskRunnerFactory {
    pub fn new_runner(trt: &TaskRunnerType) -> Box<dyn TaskRunner> {
        match trt {
            Fake => Box::new(FakeTaskRunner {}),
            LocalBlocking => Box::new(LocalTaskRunner::new())
        }
    }
}

#[cfg(test)]
#[path = "./runner_factory_test.rs"]
mod runner_factory_test;
