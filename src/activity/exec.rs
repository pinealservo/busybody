use std::path::{PathBuf};
use std::net::{SocketAddr, IpAddr};
use std::process::{Command, Stdio};
use chrono::{Utc};

use super::ActivityContext;
use super::log::{ActivityLog, ActionLogInfo, ProcessLogInfo};
use super::spec::{FileAction, IpProto};

pub fn process_action(name: &str, args: &Vec<String>, ctx: &ActivityContext) -> Result<ActivityLog, String> {
    let res = Command::new(name)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    // @todo Get a better error handling strategy
    let proc;
    match res {
        Ok(child)  => proc = child,
        Err(ioerr) => return Err("Failed to launch child".to_string()),
    }

    let mut cmdvec = vec![name.to_string()];
    cmdvec.extend(args.iter().cloned());

    let mut ctxvec = vec![ctx.cmd.to_string()];
    ctxvec.extend(ctx.args.iter().cloned());

    Ok(ActivityLog {
        timestamp: Utc::now(),
        user: ctx.user_name.to_string(),
        process: ProcessLogInfo{
            name: ctx.cmd.to_string(),
            id: ctx.pid,
            cmdline: ctxvec.join(" "),
        },
        info: ActionLogInfo::Process(ProcessLogInfo{
            name: name.to_string(),
            id: proc.id(),
            cmdline: cmdvec.join(" "),
        })
    })
}

pub fn file_action(path: &PathBuf, action: &FileAction, ctx: &ActivityContext) -> Result<ActivityLog, String> {
    Err("Not implemented yet".to_string())
}

pub fn net_action(dest: &SocketAddr, src: &IpAddr, proto: &IpProto, size: &u32, ctx: &ActivityContext) -> Result<ActivityLog, String> {
    Err("Not implemented yet".to_string())
}
