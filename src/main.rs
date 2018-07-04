extern crate serde;
extern crate serde_json;
extern crate chrono;

extern crate busybody;

use busybody::activity::*;
use busybody::native;
use std::io::{Result, Error, ErrorKind};

fn main() -> Result<()> {
    let mut cmdargs = std::env::args();

    let ctx = ActivityContext{
        user_name: native::get_username(),
        pid: std::process::id(),
        cmd: cmdargs.nth(0).unwrap().to_string(),
        args: cmdargs.collect(),
    };

    println!("Context: {:?}", ctx);

    #[cfg(windows)]
    let actspec_json = r#"[
  {
    "Process": {
      "name": "ipconfig",
      "args": ["/all"]
    }
  },
  {
    "File": {
      "path": "C:\\tmp\\foo",
      "action": {"operation": "Create", "filetype": "Directory"}
    }
  },
  {
    "File": {
      "path": "C:\\tmp\\foo\\bar.txt",
      "action": {"operation": "Create", "filetype": "File"}
    }
  },
  {
    "File": {
      "path": "C:\\tmp\\foo\\bar.txt",
      "action": {"operation": "Update", "filetype": "File"
      }
    }
  },
  {
    "File": {
      "path": "C:\\tmp\\foo\\bar.txt",
      "action": {"operation": "Delete", "filetype": "File"}
    }
  },
  {
    "File": {
      "path": "C:\\tmp\\foo",
      "action": {"operation": "Delete", "filetype": "Directory"}
    }
  },
  {
    "Network": {
      "dest": "192.168.0.1:80",
      "src": "192.168.0.134",
      "proto": "TCP",
      "size": 42
    }
  },
  {
    "Network": {
      "dest": "192.168.0.1:8000",
      "src": "192.168.0.134",
      "proto": "UDP",
      "size": 42
    }
  }
]"#;

    #[cfg(unix)]
    let actspec_json = r#"[
  {
    "Process": {
      "name": "ls",
      "args": ["-l", "/tmp"]
    }
  },
  {
    "File": {
      "path": "/tmp/foo",
      "action": {"operation": "Create", "filetype": "Directory"}
    }
  },
  {
    "File": {
      "path": "/tmp/foo/bar.txt",
      "action": {"operation": "Create", "filetype": "File"}
    }
  },
  {
    "File": {
      "path": "/tmp/foo/bar.txt",
      "action": {"operation": "Update", "filetype": "File"
      }
    }
  },
  {
    "File": {
      "path": "/tmp/foo/bar.txt",
      "action": {"operation": "Delete", "filetype": "File"}
    }
  },
  {
    "File": {
      "path": "/tmp/foo",
      "action": {"operation": "Delete", "filetype": "Directory"}
    }
  },
  {
    "Network": {
      "dest": "192.168.0.1:80",
      "src": "192.168.0.103",
      "proto": "TCP",
      "size": 42
    }
  },
  {
    "Network": {
      "dest": "192.168.0.1:8000",
      "src": "192.168.0.103",
      "proto": "UDP",
      "size": 42
    }
  }
]"#;

    let actspec_res: std::result::Result<Vec<spec::ActivitySpec>,serde_json::Error> =
        serde_json::from_str(actspec_json);
    let actspec;

    match actspec_res {
        Ok(spec) => actspec = spec,
        Err(e)   => {
            println!("Error decoding ActivitySpec from JSON: {:?}", e);
            return Err(Error::new(ErrorKind::Other, "Bad ActivitySpec".to_string()));
        }
    }

    let mut logs: Vec<log::ActivityLog> = vec![];
    for act in &actspec {
        println!("Executing ActivitySpec: {:?}", act);
        let entry = act.execute(&ctx)?;
        logs.push(entry);
    }

    println!("\nComplete logs as a JSON Array:");

    let jstr = serde_json::to_string_pretty(&logs).unwrap();
    println!("{}", jstr);

    Ok(())
}
