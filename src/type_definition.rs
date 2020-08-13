use chrono::prelude::*;
// use std::path::Path;
use std::ffi::OsString;
use uuid::Uuid;

pub type NodeId = Uuid;
pub type TaskId = Uuid;
pub type RunnerId = usize;
pub type DateTimeUtc = DateTime<Utc>;
pub type FilePath = OsString;
