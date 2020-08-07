use chrono::prelude::*;
// use std::path::Path;
use std::ffi::OsString;

pub type NodeId = usize;
pub type TaskId = usize;
pub type RunnerId = usize;
pub type DateTimeUtc = DateTime<Utc>;
pub type FilePath = OsString;
