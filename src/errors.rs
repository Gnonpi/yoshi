use crate::type_definition::NodeId;
use crate::task_definition::{TaskDefinitionType, DefinitionArgumentType};
use crossbeam_channel::TryRecvError;

/// Basic error for the crate
#[derive(Debug)]
pub enum YoshiError {
    // check dag
    DagContainsCycle,
    NoStartNode,
    
    // definition factory
    UnlinkedDefinitionType(TaskDefinitionType),
    
    // creating and running definitions
    MissingDefinitionArgumentEntry(String),
    WrongTypeDefinitionArgumentEntry(String, DefinitionArgumentType),
    TaskDefinitionRunFailure(String),
    
    // adding elements to dag
    AddingNodeWithUnknownParent(NodeId),
    AddingNodeWithUnknownChild(NodeId),

    AddingEdgeWithUnknownParent(NodeId),
    AddingEdgeWithUnknownChild(NodeId),
    
    // running dag
    NodeFailedToRun(NodeId, String),
    ErrorWhileReadingFromRunnerChannel(TryRecvError),
    CompletedNodeWithoutInstance,

    // parsing dag
    ParsingNoNodes,
    ParsingNodeIncompatibleEntries {
        node_label: String,
        entry_one: String,
        entry_two: String
    },
    ParsingNodeUnknownRefDefinition {
        node_label: String,
        ref_definition: String
    },
    ParsingNodeUnknownRefRunner {
        node_label: String,
        ref_runner: String
    },
    ParsingEdgeUnknownParent(String),
    ParsingEdgeOrphanNode {
        parent_id: String,
        child_id: String
    },
    UnknownConfigSuffix(String),
    CannotValidateConfig,

    // dev
    ToImplementSoon
}
