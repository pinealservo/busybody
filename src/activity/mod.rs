use serde_json;

pub mod spec;
pub mod log;
#[cfg(not(test))]
pub mod exec;
#[cfg(test)]
pub mod fake_exec;

/// Contains contextual data about the activity to include with the log
#[derive(Debug)]
pub struct ActivityContext {
    pub user_name: String,
    pub pid: u32,
    pub cmd: String,
    pub args: Vec<String>,
}

pub fn print_activity_info(act: &log::ActivityLog) -> Result<(), serde_json::Error> {
    let jstr = serde_json::to_string(act)?;
    println!("{}", jstr);
    Ok(())
}
