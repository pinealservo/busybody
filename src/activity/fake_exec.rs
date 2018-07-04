use std::path::{self, PathBuf};
use std::net::{self, SocketAddr, IpAddr};

use super::ActivityContext;
use super::log::ActivityLog;
use super::spec::{FileAction, IpProto};

pub fn process_action(name: &str, args: &Vec<String>, ctx: &ActivityContext) -> Result<ActivityLog, String> {
    Err("Not implemented yet".to_string())
}

pub fn file_action(path: &PathBuf, action: &FileAction, ctx: &ActivityContext) -> Result<ActivityLog, String> {
    Err("Not implemented yet".to_string())
}

pub fn net_action(dest: &SocketAddr, src: &IpAddr, proto: &IpProto, size: &u32, ctx: &ActivityContext) -> Result<ActivityLog, String> {
    Err("Not implemented yet".to_string())
}
