use crate::errors::YoshiError;
use crate::task_definition::{
    generate_task_definition_id, 
    TaskDefinition, 
    TaskDefinitionType
};
use crate::task_definition::DefinitionArguments;
use std::convert::TryFrom;
use crate::task_output::TaskOutput;
use crate::type_definition::TaskId;
use log::{debug, info, error};
use std::panic;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RustFunctionTaskDefinition {
    task_def_id: TaskId,
    callable: fn()
}

impl RustFunctionTaskDefinition {
    fn run_callable(&self) {
        (self.callable)()
    }

    fn new(func: fn()) -> Self {
        RustFunctionTaskDefinition {
            task_def_id: generate_task_definition_id(),
            callable: func
        }
    }
}

fn my_call() {
    println!("gallo");
}

impl TryFrom<DefinitionArguments> for RustFunctionTaskDefinition {
    type Error = YoshiError;

    fn try_from(da: DefinitionArguments) -> Result<Self, Self::Error> {
        Ok(RustFunctionTaskDefinition {
            task_def_id: generate_task_definition_id(),
            callable: my_call
        })
    }
}

impl TaskDefinition for RustFunctionTaskDefinition {
    fn task_definition_id(&self) -> TaskId {
        self.task_def_id
    }

    fn task_type(&self) -> TaskDefinitionType {
        TaskDefinitionType::RustFunction
    }

    fn run(&self) -> Result<TaskOutput, YoshiError> {
        info!("Starting running function");
        // not the recommended way to catch those panics
        match panic::catch_unwind(|| {
            debug!("Inside catch_unwind block");
            self.run_callable();
        }) {
            Ok(()) => return Ok(TaskOutput::Nothing),
            Err(err) => {
                let err_msg = format!("Rust function panicked with: {:?}", err); 
                error!("{}", err_msg);
                return Err(YoshiError::TaskDefinitionRunFailure(err_msg))
            }
        }
    }

    fn get_params(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}


#[cfg(test)]
#[path = "./rust_func_task_test.rs"]
mod rust_func_task_test;
