#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate killjp.
//!
//! add doc here

use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::process::Command;
use std::str::FromStr;

use glob::Pattern;

/// Pid
#[derive(Debug)]
pub struct Pid {
    /// value
    pub value: u32,
}

impl Pid {
    fn new(value: u32) -> Self {
        Self { value }
    }
}

impl Deref for Pid {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FromStr for Pid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pid::new(s.parse()?))
    }
}

impl Display for Pid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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
        .map(|x| Proc { pid: x.split(' ').next().expect("").parse().expect("parse to pid"), detail: x.to_string() })
        .collect();

    if let Some(pattern) = glob {
        let p = Pattern::new(&pattern).expect("pattern");
        pid_list.retain(|x| p.matches(&x.detail));
    }

    Ok(pid_list)
}

/// kill process by pid.
pub fn kill_all(pid_list: Vec<Pid>, force: bool) -> anyhow::Result<()> {
    let (cmd_string, mut command) = to_kill_command(pid_list, force);
    println!("exec: {}", cmd_string);
    println!("{}", "-".repeat(60));

    let result = command.output()?;
    println!("{}", result.status);
    println!("stdout: {}", String::from_utf8_lossy(&result.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&result.stderr));

    Ok(())
}

fn to_kill_command(pid_list: Vec<Pid>, force: bool) -> (String, Command) {
    assert!(!pid_list.is_empty());

    let args = {
        let mut args = Vec::new();
        args.push("-c".to_string());

        let mut sub_args = Vec::new();
        sub_args.push("kill".to_string());
        if force {
            sub_args.push("-f".to_string());
        }
        for pid in pid_list {
            sub_args.push(pid.to_string());
        }

        let kill = sub_args.into_iter().collect::<Vec<String>>().join(" ");
        args.push(kill);
        args
    };

    let args_for_display = args
        .iter()
        .map(|x| if x.contains(' ') { format!("'{x}'") } else { x.to_string() })
        .collect::<Vec<String>>()
        .join(" ");

    let cmd_string = format!("nu {}", args_for_display);

    let mut nu = Command::new("nu");
    nu.args(args);

    (cmd_string, nu)
}
