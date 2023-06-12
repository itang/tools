#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate killjp.
//!
//! add doc here

use std::process::Command;

/// Pid type
pub type Pid = u32;

/// get java process list
pub fn get_java_proces_list() -> anyhow::Result<Vec<Pid>> {
    let output = Command::new("jps").args(vec!["-lv"]).output()?;

    let result = String::from_utf8(output.stdout)?;
    println!("{}", result);

    let pid_list: Vec<u32> = result
        .lines()
        .filter(|x| !x.contains("jps"))
        .map(|x| x.split(' ').next().expect("").to_string().parse().expect(""))
        .collect();

    Ok(pid_list)
}

/// kill process by pid.
pub fn kill_all(pid_list: Vec<u32>, force: bool) -> anyhow::Result<()> {
    let result = to_kill_command(pid_list, force).output()?;
    println!("{:?}", result);

    Ok(())
}

fn to_kill_command(pid_list: Vec<u32>, force: bool) -> Command {
    assert!(!pid_list.is_empty());

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
