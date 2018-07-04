extern crate serde;
extern crate serde_json;
extern crate chrono;

extern crate libbusybody;

use std::path::PathBuf;
use std::net::{IpAddr, SocketAddr};
use chrono::prelude::*;

use libbusybody::activity::*;

fn main() {
    println!("Hello, world!");
    let testproc = ProcessLogInfo {
        name: "test".to_owned(),
        id: 42,
        cmdline: "test 1 2 3".to_owned(),
    };

    print_process_info(&testproc).unwrap();

    let testact = ActivityLog{
        timestamp: Utc::now(),
        user: "joe".to_owned(),
        process: ProcessLogInfo{
            name: "busybody".to_owned(),
            id: 42,
            cmdline: "busybody runall".to_owned()
        },
        info: ActionLogInfo::File(FileLogInfo{
            path: ["/", "Users", "joe", "note.txt"].iter().collect(),
            operation: FileAction::Create,
        }),
    };

    print_activity_info(&testact).unwrap();
}
