extern crate serde;
extern crate serde_json;
extern crate chrono;

extern crate libbusybody;

use libbusybody::activity::*;
use libbusybody::native;

fn main() {
    println!("Hello, world!");

    let mut cmdargs = std::env::args();

    let ctx = ActivityContext{
        user_name: native::get_username(),
        pid: std::process::id(),
        cmd: cmdargs.nth(0).unwrap().to_string(),
        args: cmdargs.collect(),
    };

    println!("Context: {:?}", ctx);

    let spec = spec::ActivitySpec::Process{
        name: "ls".to_string(),
        args: vec!["-l".to_string()],
    };

    println!("ActivitySpec: {:?}", spec);

    let result = spec.execute(&ctx);
    match result {
        Ok(entry)   => print_activity_info(&entry).unwrap(),
        Err(errstr) => println!("{}", errstr),
    };

}
