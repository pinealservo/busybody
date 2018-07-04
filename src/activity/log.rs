use super::spec::*;

use std::net::SocketAddr;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

/// Information to log about a process
#[derive(Serialize,Debug)]
pub struct ProcessLogInfo {
    pub name: String,
    pub id: u32,
    pub cmdline: String,
}

/// Information to log about a file operation
#[derive(Serialize,Debug)]
pub struct FileLogInfo {
    pub path: PathBuf,
    pub operation: FileAction,
}

/// Information to log about a network operation
#[derive(Serialize,Debug)]
pub struct NetLogInfo {
    pub dest: SocketAddr,
    pub src: SocketAddr,
    pub proto: IpProto,
    pub size: u32,
}

/// Action-specific information to log
#[derive(Serialize,Debug)]
pub enum ActionLogInfo {
    Process(ProcessLogInfo),
    File(FileLogInfo),
    Network(NetLogInfo),
}

/// A single log entry for an action
#[derive(Serialize,Debug)]
pub struct ActivityLog {
    pub timestamp: DateTime<Utc>,
    pub user: String,
    pub process: ProcessLogInfo,
    pub info: ActionLogInfo,
}
