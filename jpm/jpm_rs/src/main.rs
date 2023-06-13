#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use clap::{Args, Parser, Subcommand};

use jpm::{get_java_proces_list, kill_all, Proc};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    List(ListArgs),
    Kill(KillArgs),
}

#[derive(Args, Debug)]
struct KillArgs {
    /// The glob pattern
    #[arg(short, long)]
    glob: Option<String>,
}

#[derive(Args, Debug)]
struct ListArgs {
    /// The glob pattern
    #[arg(short, long)]
    glob: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.action {
        Action::List(args) => handle_list(args)?,
        Action::Kill(args) => handle_kill(args)?,
    }

    Ok(())
}

fn handle_kill(args: KillArgs) -> anyhow::Result<()> {
    let pid_list: Vec<Proc> = get_java_proces_list(args.glob)?;
    display(&pid_list);

    if pid_list.is_empty() {
        println!("INFO: no java process, exit")
    } else {
        kill_all(pid_list.into_iter().map(|x| x.pid).collect(), true)?;
    }

    Ok(())
}

fn handle_list(args: ListArgs) -> anyhow::Result<()> {
    let pid_list: Vec<Proc> = get_java_proces_list(args.glob)?;
    display(&pid_list);

    println!("INFO: java process pid list: {:?}", pid_list.iter().map(|x| *x.pid).collect::<Vec<u32>>());
    println!(
        "INFO: for kill cmd, kill -f {}",
        pid_list.iter().map(|x| x.pid.0.to_string()).collect::<Vec<String>>().join(" ")
    );

    Ok(())
}

fn display(procs: &[Proc]) {
    for p in procs {
        println!("{}", p.detail);
    }
}
