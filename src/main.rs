extern crate serde;
extern crate serde_json;
extern crate chrono;

extern crate busybody;

use busybody::activity::*;
use busybody::native;
use std::io::Result;

fn main() -> Result<()> {
    let mut cmdargs = std::env::args();

    let ctx = ActivityContext{
        user_name: native::get_username(),
        pid: std::process::id(),
        cmd: cmdargs.nth(0).unwrap().to_string(),
        args: cmdargs.collect(),
    };

    println!("Context: {:?}", ctx);

    let mut acts: Vec<spec::ActivitySpec> = vec![];

    acts.push(spec::ActivitySpec::Process{
        name: "ls".to_string(),
        args: vec!["-l".to_string(), "/tmp".to_string()],
    });
    acts.push(spec::ActivitySpec::File{
        path: ["/", "tmp", "foo"].into_iter().collect(),
        action: spec::FileAction{ operation: spec::FileOp::Create, filetype: spec::FileType::Directory },
    });
    acts.push(spec::ActivitySpec::File{
        path: ["/", "tmp", "foo", "bar.txt"].into_iter().collect(),
        action: spec::FileAction{ operation: spec::FileOp::Create, filetype: spec::FileType::File },
    });
    acts.push(spec::ActivitySpec::File{
        path: ["/", "tmp", "foo", "bar.txt"].into_iter().collect(),
        action: spec::FileAction{ operation: spec::FileOp::Update, filetype: spec::FileType::File },
    });
    acts.push(spec::ActivitySpec::File{
        path: ["/", "tmp", "foo", "bar.txt"].into_iter().collect(),
        action: spec::FileAction{ operation: spec::FileOp::Delete, filetype: spec::FileType::File },
    });
    acts.push(spec::ActivitySpec::File{
        path: ["/", "tmp", "foo"].into_iter().collect(),
        action: spec::FileAction{ operation: spec::FileOp::Delete, filetype: spec::FileType::Directory },
    });
    acts.push(spec::ActivitySpec::Network{
        dest: "192.168.0.1:80".parse().unwrap(),
        src: "192.168.0.103".parse().unwrap(),
        proto: spec::IpProto::TCP,
        size: 42,
    });
    acts.push(spec::ActivitySpec::Network{
        dest: "192.168.0.1:8000".parse().unwrap(),
        src: "192.168.0.103".parse().unwrap(),
        proto: spec::IpProto::UDP,
        size: 42,
    });

    let mut logs: Vec<log::ActivityLog> = vec![];
    for act in &acts {
        println!("\nActivitySpec: {:?}\n", act);
        let entry = act.execute(&ctx)?;
        logs.push(entry);
    }

    let jstr = serde_json::to_string(&logs).unwrap();
    println!("{}", jstr);

    Ok(())
}
