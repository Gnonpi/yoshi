use crate::runners::{TaskRunner, MessageToRunner, MessageFromRunner};
use crate::task_output::TaskOutput;
use crate::task_instance::{TaskStatus, TaskInstance};
use crate::task_definition::TaskDefinition;
use crate::type_definition::{NodeId, RunnerId, DateTimeUtc};
use crossbeam_channel::{unbounded, Sender, Receiver};
use chrono::prelude::*;


#[derive(Debug, Clone)]
pub struct LocalTaskRunner {    
    date_started: Option<DateTimeUtc>,
    date_finished: Option<DateTimeUtc>
}

impl TaskRunner for LocalTaskRunner {
    fn get_runner_id(&self) -> RunnerId {
        return 1
    }
    fn start_task(
        &self,
        node_id: NodeId,
        task_def: &dyn TaskDefinition,
    ) -> (Sender<MessageToRunner>, Receiver<MessageFromRunner>) {
        let (s, _) = unbounded::<MessageToRunner>(); 
        let (_, r) = unbounded::<MessageFromRunner>(); 
        (s, r)
    }
    fn get_status(&self) -> TaskStatus {
        TaskStatus::Success
    }
    fn get_task_instance(&self) -> Option<TaskInstance> {
        let inst = TaskInstance {
            id_node: NodeId::new_v4(),
            id_task_definition: NodeId::new_v4(),
            id_task_runner: self.get_runner_id(),
            date_started: Utc::now(),
            date_finished: Utc::now(),
            status: TaskStatus::Success,
            output: TaskOutput::Text("ok".to_string())
        };
        Some(inst)
    }
}
