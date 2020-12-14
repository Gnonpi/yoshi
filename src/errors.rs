use crate::type_definition::NodeId;
use crate::task_definition::{TaskDefinitionType, DefinitionArgumentType};
use crossbeam_channel::TryRecvError;

/// Basic error for the crate
#[derive(Debug)]
pub enum YoshiError {
    DagContainsCycle,
    NoStartNode,
    
    UnlinkedDefinitionType(TaskDefinitionType),
    
    MissingDefinitionArgumentEntry(String),
    WrongTypeDefinitionArgumentEntry(String, DefinitionArgumentType),
    TaskDefinitionRunFailure(String),
    
    AddingNodeWithUnknownParent(NodeId),
    AddingNodeWithUnknownChild(NodeId),

    AddingEdgeWithUnknownParent(NodeId),
    AddingEdgeWithUnknownChild(NodeId),
    
    NodeFailedToRun(NodeId, String),
    ErrorWhileReadingFromRunnerChannel(TryRecvError),
    CompletedNodeWithoutInstance,
}
