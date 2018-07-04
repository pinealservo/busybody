///! # libbusybody
///!
///! `libbusybody` is a toolset for generating potentially interesting system activity

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
mod windows;
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
