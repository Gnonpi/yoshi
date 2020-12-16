use crate::task_definition::{DefinitionArgumentType, TaskDefinitionType};
use crate::type_definition::NodeId;
use crossbeam_channel::TryRecvError;
use std::error::Error;
use std::fmt;

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
        entry_two: String,
    },
    ParsingNodeUnknownRefDefinition {
        node_label: String,
        ref_definition: String,
    },
    ParsingNodeUnknownRefRunner {
        node_label: String,
        ref_runner: String,
    },
    ParsingEdgeUnknownParent(String),
    ParsingEdgeOrphanNode {
        parent_id: String,
        child_id: String,
    },
    UnknownConfigSuffix(String),
    CannotValidateConfig,

    // dev
    ToImplementSoon,
}

impl fmt::Display for YoshiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // todo: implement true error messages for YoshiError
        // let's use Debug for now
        write!(f, "Error: {:?}", self)
    }
}

impl Error for YoshiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // todo: split into subcategories
        // We don't have sub-errors and implementing all sources is a bit complicating with so much
        // we'll do it when splitting YoshiError
        None
    }
}
