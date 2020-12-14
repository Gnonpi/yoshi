use crate::type_definition::NodeId;
use crate::task_definition::TaskDefinitionType;
use crossbeam_channel::TryRecvError;

/// Basic error for the crate
#[derive(Debug)]
pub enum YoshiError {
    // dag_checker
    DagContainsCycle,
    // dag.run
    NoStartNode,
    // task definition
    MissingDefinitionArgumentEntry,
    WrongTypeDefinitionArgumentEntry,
    TaskDefinitionRunFailure(String),
    // dag.add_task & dag.add_edge
    AddingNodeWithUnknownParent,
    AddingNodeWithUnknownChildren,
    // dag.run taskrunner
    NodeFailedToRun(NodeId, String),
    ErrorWhileReadingFromRunnerChannel(TryRecvError),
    CompletedNodeWithoutInstance,
    // definition factory
    UnlinkedDefinitionType(TaskDefinitionType),
}
