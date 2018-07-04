use std::path::{PathBuf};
use std::net::{SocketAddr, IpAddr, TcpStream, UdpSocket};
use std::process::{Command, Stdio};
use std::fs::{self, File, OpenOptions};
use std::io::{Result, Error, ErrorKind, Write};
use std::iter;
use chrono::{Utc};

use super::ActivityContext;
use super::log::{ActivityLog, ActionLogInfo, ProcessLogInfo, FileLogInfo, NetLogInfo};
use super::spec::{FileAction, FileOp, FileType, IpProto};

pub fn process_action(name: &str, args: &Vec<String>, ctx: &ActivityContext) -> Result<ActivityLog> {
    let proc = Command::new(name)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

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

pub fn file_action(path: &PathBuf, action: &FileAction, ctx: &ActivityContext) -> Result<ActivityLog> {
    match &action {
        FileAction{ operation: FileOp::Create, filetype: FileType::File } => {
            File::create(path)?;
        },
        FileAction{ operation: FileOp::Update, filetype: FileType::File } => {
            let mut f = OpenOptions::new()
                .write(true)
                .open(path)?;
            f.write_all(b"Hello, World!\n")?;
            f.sync_data()?;
        },
        FileAction{ operation: FileOp::Delete, filetype: FileType::File } => {
            fs::remove_file(path)?;
        },
        FileAction{ operation: FileOp::Create, filetype: FileType::Directory } => {
            fs::create_dir(path)?;
        },
        FileAction{ operation: FileOp::Update, filetype: FileType::Directory } => {
            return Err(Error::new(ErrorKind::Other, "Not sure what updating a directory should do".to_string()))
        },
        FileAction{ operation: FileOp::Delete, filetype: FileType::Directory } => {
            fs::remove_dir(path)?;
        },
    };

    let mut ctxvec = vec![ctx.cmd.to_string()];
    ctxvec.extend(ctx.args.iter().cloned());

    Ok(ActivityLog{
        timestamp: Utc::now(),
        user: ctx.user_name.to_string(),
        process: ProcessLogInfo{ name: ctx.cmd.to_string(), id: ctx.pid, cmdline: ctxvec.join(" ") },
        info: ActionLogInfo::File(FileLogInfo{
            path: path.to_path_buf(),
            operation: action.clone()
        })
    })
}

pub fn net_action(dest: &SocketAddr, src: &IpAddr, proto: &IpProto, size: &u32, ctx: &ActivityContext) -> Result<ActivityLog> {
    let srcsock: SocketAddr;
    let buf: Vec<u8> = iter::repeat('a' as u8).take(*size as usize).collect();

    match &proto {
        IpProto::TCP => {
            let mut stream = TcpStream::connect(dest)?;
            srcsock = stream.local_addr()?;
            stream.write(&buf)?;
        },
        IpProto::UDP => {
            let mut sock = UdpSocket::bind((*src, dest.port()))?;
            srcsock = SocketAddr::from((*src, dest.port()));
            sock.send_to(&buf, dest)?;
        },
    };

    let mut ctxvec = vec![ctx.cmd.to_string()];
    ctxvec.extend(ctx.args.iter().cloned());

    Ok(ActivityLog{
        timestamp: Utc::now(),
        user: ctx.user_name.to_string(),
        process: ProcessLogInfo{ name: ctx.cmd.to_string(), id: ctx.pid, cmdline: ctxvec.join(" ") },
        info: ActionLogInfo::Network(NetLogInfo{
            dest: dest.clone(),
            src: srcsock,
            proto: proto.clone(),
            size: *size,
        })
    })
}
