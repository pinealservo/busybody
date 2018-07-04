//! # busybody
//!
//! `busybody` is a toolset for generating potentially interesting system activity
//!
//! ## Supported Activity Types
//!
//! + Process Creation
//! + File Creation, Modification, Deletion
//! + TCP/IP Connection + Transmission
//!
//! ## Logged Information
//!
//! For each activity, busybody records a log entry. These are JSON objects with the
//! following information:
//!
//! ### Common Fields
//!
//! + Event Timestamp
//! + Username of the controlling user
//! + Process Record
//!     - Process Name
//!     - Process ID
//!     - Process Command Line
//! + Event Type
//!
//! ### Process Creation Specific Fields
//!
//! + New Process Record
//!     - Process Name
//!     - Process ID
//!     - Process Command Line
//!
//! ### File Operation Specific Fields
//!
//! + Full path name to the file operated on
//! + Operation Type (Create, Modify, Delete)
//! + Filesystem Object Type (File, Directory)
//!
//! ### Network Operation Specific Fields
//!
//! + Destination IP Address and Port
//! + Source IP Address and Port
//! + Protocol Name
//! + Byte count of transmitted data

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
#[cfg(unix)]
extern crate libc;

pub mod activity;

#[cfg(unix)]
pub mod unix;
#[cfg(unix)]
pub use self::unix as native;

#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use self::windows as native;

#[cfg(test)]
mod tests {
    use activity::*;
    use activity::log::*;
    use activity::spec::*;
    use super::native;

    use chrono::prelude::*;

    #[test]
    fn printing_file_activity_works() {
        let testact = ActivityLog{
            timestamp: Utc::now(),
            user: native::get_username(),
            process: ProcessLogInfo{
                name: "busybody".to_owned(),
                id: 42,
                cmdline: "busybody runall".to_owned()
            },
            info: ActionLogInfo::File(FileLogInfo{
                path: ["/", "Users", "joe", "note.txt"].iter().collect(),
                operation: FileAction{
                    operation: FileOp::Create,
                    filetype: FileType::File,
                },
            }),
        };

        print_activity_info(&testact).unwrap();
    }
}
