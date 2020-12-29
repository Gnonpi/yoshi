mod bash_task;
mod definition_arguments;
mod definition_factory;
mod dummy_task;
mod python_task;
mod task_def;

pub use bash_task::BashTaskDefinition;
pub use definition_arguments::{
    DefinitionArgumentElement, DefinitionArgumentType, DefinitionArguments,
};
pub use definition_factory::{
    create_new_definition, string_to_definition_type, TaskDefinitionType,
};
pub use dummy_task::DummyTaskDefinition;
pub use python_task::PythonTaskDefinition;
pub use task_def::{generate_task_definition_id, TaskDefinition};

// #[cfg(feature = "db_sqlite")]
mod db;
// #[cfg(feature = "db_sqlite")]
pub use db::*;
