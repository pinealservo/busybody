///! # libbusybody
///!
///! `libbusybody` is a toolset for generating potentially interesting system activity

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate chrono;

pub mod activity {
    use serde_json;
    use std::path::PathBuf;
    use std::net::{IpAddr, SocketAddr};
    use chrono::prelude::*;

    /// Specifies the type of file operation to perform
    #[derive(Serialize, Deserialize)]
    pub enum FileAction {
        Create,
        Update,
        Delete,
    }

    /// Specifies which IP protocol to use for the network operation
    #[derive(Serialize, Deserialize)]
    pub enum IpProto {
        TCP,
        UDP,
    }

    /// Specifies the activity to perform
    #[derive(Serialize, Deserialize)]
    pub enum ActivitySpec {
        Process{ name: String, args: Vec<String> },
        File{ path: PathBuf, action: FileAction },
        Network{ dest: SocketAddr, src: IpAddr, proto: IpProto, size: u32 },
    }

    /// Contains contextual data about the activity to include with the log
    pub struct ActivityContext {
        pub user_name: String,
        pub pid: u32,
        pub cmdline: String,
    }

    /// Information to log about a process
    #[derive(Serialize)]
    pub struct ProcessLogInfo {
        pub name: String,
        pub id: i32,
        pub cmdline: String,
    }

    /// Information to log about a file operation
    #[derive(Serialize)]
    pub struct FileLogInfo {
        pub path: PathBuf,
        pub operation: FileAction,
    }

    /// Information to log about a network operation
    #[derive(Serialize)]
    pub struct NetLogInfo {
        pub dest: SocketAddr,
        pub src: SocketAddr,
        pub proto: IpProto,
        pub size: u32,
    }

    /// Action-specific information to log
    #[derive(Serialize)]
    pub enum ActionLogInfo {
        Process(ProcessLogInfo),
        File(FileLogInfo),
        Network(NetLogInfo),
    }

    /// A single log entry for an action
    #[derive(Serialize)]
    pub struct ActivityLog {
        pub timestamp: DateTime<Utc>,
        pub user: String,
        pub process: ProcessLogInfo,
        pub info: ActionLogInfo,
    }

    impl ActivitySpec {
        /// Execute an ActivitySpec with an ActivityContext and return the resulting log entry
        pub fn execute(&self, ctx : &ActivityContext) -> Result<ActivityLog, String> {
            Err("Not implemented yet".to_owned())
        }
    }

    pub fn print_activity_info(act : &ActivityLog) -> Result<(), serde_json::Error> {
        let jstr = serde_json::to_string(act)?;
        println!("{}", jstr);
        Ok(())
    }

    pub fn print_process_info(pi : &ProcessLogInfo) -> Result<(), serde_json::Error> {
        let jstr = serde_json::to_string(pi)?;
        println!("{}", jstr);
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use activity::*;
    use std::path::PathBuf;
    use std::net::{IpAddr, SocketAddr};
    use chrono::prelude::*;

    #[test]
    fn printing_process_info_works() {
        let testproc = ProcessLogInfo {
            name: "test".to_owned(),
            id: 42,
            cmdline: "test 1 2 3".to_owned(),
        };

        print_process_info(&testproc).unwrap();
    }

    #[test]
    fn printing_file_activity_works() {
        let testact = ActivityLog{
            timestamp: Utc::now(),
            user: "joe".to_owned(),
            process: ProcessLogInfo{
                name: "busybody".to_owned(),
                id: 42,
                cmdline: "busybody runall".to_owned()
            },
            info: ActionLogInfo::File(FileLogInfo{
                path: ["C:\\", "Users", "joe", "note.txt"].iter().collect(),
                operation: FileAction::Create,
            }),
        };

        print_activity_info(&testact).unwrap();
    }
}
