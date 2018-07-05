use std::path::{PathBuf};
use std::net::{SocketAddr, IpAddr};
use std::io::{Error, ErrorKind};
use std::io::Result;

use super::ActivityContext;
use super::log::ActivityLog;
use super::spec::{FileAction, IpProto};

pub fn process_action(_name: &str, _args: &Vec<String>, _ctx: &ActivityContext) -> Result<ActivityLog> {
    Err(Error::new(ErrorKind::Other, "Not implemented yet".to_string()))
}

pub fn file_action(_path: &PathBuf, _action: &FileAction, _ctx: &ActivityContext) -> Result<ActivityLog> {
    Err(Error::new(ErrorKind::Other, "Not implemented yet".to_string()))
}

pub fn net_action(_dest: &SocketAddr, _src: &IpAddr, _proto: &IpProto, _size: &u32, _ctx: &ActivityContext) -> Result<ActivityLog> {
    Err(Error::new(ErrorKind::Other, "Not implemented yet".to_string()))
}
