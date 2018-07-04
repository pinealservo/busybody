use super::ActivityContext;
use super::log::ActivityLog;
#[cfg(not(test))]
use super::exec;
#[cfg(test)]
use super::fake_exec as exec;

use std::path::PathBuf;
use std::net::{IpAddr, SocketAddr};

/// Specifies the type of file operation to perform
#[derive(Serialize, Deserialize, Debug)]
pub enum FileAction {
    Create,
    Update,
    Delete,
}

/// Specifies which IP protocol to use for the network operation
#[derive(Serialize, Deserialize, Debug)]
pub enum IpProto {
    TCP,
    UDP,
}

/// Specifies the activity to perform
#[derive(Serialize, Deserialize, Debug)]
pub enum ActivitySpec {
    Process{ name: String, args: Vec<String> },
    File{ path: PathBuf, action: FileAction },
    Network{ dest: SocketAddr, src: IpAddr, proto: IpProto, size: u32 },
}

impl ActivitySpec {
    /// Execute an ActivitySpec with an ActivityContext and return the resulting log entry
    pub fn execute(&self, ctx : &ActivityContext) -> Result<ActivityLog, String> {
        use self::ActivitySpec::*;
        match self {
            Process{ name, args } => exec::process_action(name, args, ctx),
            File{ path, action }  => exec::file_action(path, action, ctx),
            Network{ dest, src, proto, size } => exec::net_action(dest, src, proto, size, ctx),
        }
    }
}
