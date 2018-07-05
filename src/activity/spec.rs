use super::ActivityContext;
use super::log::ActivityLog;
#[cfg(not(test))]
use super::exec;
#[cfg(test)]
use super::fake_exec as exec;

use std::path::PathBuf;
use std::net::{IpAddr, SocketAddr};
use std::io::Result;

/// Specifies the type of file operation to perform
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FileOp {
    Create,
    Update,
    Delete,
}

/// Specifies the kind of filesystem object to operate on
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FileType {
    File,
    Directory,
}

/// Specifies a file action to perform
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileAction {
    pub operation: FileOp,
    pub filetype: FileType,
}

/// Specifies which IP protocol to use for the network operation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IpProto {
    TCP,
    UDP,
}

/// Specifies the activity to perform
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActivitySpec {
    Process{ name: String, args: Vec<String> },
    File{ path: PathBuf, action: FileAction },
    Network{ dest: SocketAddr, src: IpAddr, proto: IpProto, size: u32 },
}

impl ActivitySpec {
    /// Execute an ActivitySpec with an ActivityContext and return the resulting log entry
    pub fn execute(&self, ctx : &ActivityContext) -> Result<ActivityLog> {
        use self::ActivitySpec::*;
        match self {
            Process{ name, args } => exec::process_action(name, args, ctx),
            File{ path, action }  => exec::file_action(path, action, ctx),
            Network{ dest, src, proto, size } => exec::net_action(dest, src, proto, size, ctx),
        }
    }
}
