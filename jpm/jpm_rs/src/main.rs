#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use clap::{Args, Parser, Subcommand};

use jpm::{get_java_proces_list, kill_all};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    List,
    Kill(KillArgs),
}

#[derive(Args, Debug)]
struct KillArgs {}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.action {
        Action::List => handle_list()?,
        Action::Kill(args) => handle_kill(args)?,
    }

    Ok(())
}

fn handle_kill(_args: KillArgs) -> anyhow::Result<()> {
    let pid_list: Vec<u32> = get_java_proces_list()?;
    if pid_list.is_empty() {
        println!("INFO: no java process, exit")
    } else {
        kill_all(pid_list, true)?;
    }
    Ok(())
}

fn handle_list() -> anyhow::Result<()> {
    let pid_list: Vec<u32> = get_java_proces_list()?;

    println!("INFO: java process pid list: {:?}", pid_list);
    println!(
        "INFO: for kill cmd, kill -f {}",
        pid_list.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")
    );

    Ok(())
}
