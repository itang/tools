#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate killjp.
//!
//! add doc here

use std::ops::Deref;
use std::process::Command;

use glob::Pattern;

/// Pid
#[derive(Debug)]
pub struct Pid(pub u32);

impl Deref for Pid {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Proc type
#[derive(Debug)]
pub struct Proc {
    /// pid
    pub pid: Pid,
    /// detail info
    pub detail: String,
}

/// get java process list
pub fn get_java_proces_list(glob: Option<String>) -> anyhow::Result<Vec<Proc>> {
    let output = Command::new("jps").args(vec!["-lv"]).output()?;

    let result = String::from_utf8(output.stdout)?;

    let mut pid_list: Vec<Proc> = result
        .lines()
        .filter(|x| !x.contains("jps"))
        .map(|x| Proc {
            pid: Pid(x.split(' ').next().expect("").to_string().parse().expect("")),
            detail: x.to_string(),
        })
        .collect();

    if let Some(pattern) = glob {
        let p = Pattern::new(&pattern).expect("pattern");
        pid_list.retain(|x| p.matches(&x.detail));
    }

    Ok(pid_list)
}

/// kill process by pid.
pub fn kill_all(pid_list: Vec<Pid>, force: bool) -> anyhow::Result<()> {
    let result = to_kill_command(pid_list, force).output()?;
    println!("exit status: {}", result.status);
    println!("stdout: {}", String::from_utf8_lossy(&result.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&result.stderr));

    Ok(())
}

fn to_kill_command(pid_list: Vec<Pid>, force: bool) -> Command {
    assert!(!pid_list.is_empty());

    let mut args = Vec::new();
    args.push("-c".to_string());

    let mut sub_args = Vec::new();
    sub_args.push("kill".to_string());
    if force {
        sub_args.push("-f".to_string());
    }
    for pid in pid_list {
        sub_args.push(pid.0.to_string());
    }

    let kill = sub_args.into_iter().collect::<Vec<String>>().join(" ");
    args.push(kill);

    let args_for_display = args
        .iter()
        .map(|x| if x.contains(' ') { format!("'{x}'") } else { x.to_string() })
        .collect::<Vec<String>>()
        .join(" ");

    println!("exec: nu {}", args_for_display);

    let mut nu = Command::new("nu");
    nu.args(args);

    nu
}
