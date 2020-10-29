use crate::runners::{TaskRunnerFactory, TaskRunnerType, FakeTaskRunner};

#[test]
fn it_can_return_a_runner() {
    let runner = TaskRunnerFactory::new_runner(&TaskRunnerType::Fake);
    assert_eq!(runner.get_runner_id(), TaskRunnerType::Fake);

    let local_runner = TaskRunnerFactory::new_runner(&TaskRunnerType::LocalBlocking);
    assert_eq!(local_runner.get_runner_id(), TaskRunnerType::LocalBlocking);
}
