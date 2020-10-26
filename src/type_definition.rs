use chrono::prelude::*;
// use std::path::Path;
use std::ffi::OsString;
use uuid::Uuid;
use crate::runners::TaskRunnerType;

pub type NodeId = Uuid;
pub type TaskId = Uuid;
pub type RunnerId = TaskRunnerType;
pub type DateTimeUtc = DateTime<Utc>;
pub type FilePath = OsString;
