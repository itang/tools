#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use clap::{Args, Parser, Subcommand};
use colored::Colorize;

use jpm::Proc;

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

    /// force kill
    #[arg(short, long)]
    force: bool,
    //TODO: 指定 pids for kill
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
    let pid_list: Vec<Proc> = Proc::get_java_process_list(args.glob)?;

    display(&pid_list);

    if pid_list.is_empty() {
        println!("INFO: No Found Java process, just exit.")
    } else {
        println!("{}", "-".repeat(60));
        Proc::kill_all(pid_list.into_iter().map(|x| x.pid).collect(), args.force)?;
    }

    Ok(())
}

fn handle_list(args: ListArgs) -> anyhow::Result<()> {
    let pid_list: Vec<Proc> = Proc::get_java_process_list(args.glob)?;

    display(&pid_list);

    if pid_list.is_empty() {
        println!("INFO: No Found Java process, just exit.")
    } else {
        println!("{}", "-".repeat(60));
        let pids = pid_list.iter().map(|x| x.pid.to_string().green().to_string()).collect::<Vec<String>>().join(", ");
        println!("INFO: Java process pid list: [{}]", pids);
        println!(
            "INFO: kill cmd, 'kill -f {}'",
            pid_list.iter().map(|x| x.pid.to_string()).collect::<Vec<String>>().join(" ").green()
        );
    }

    Ok(())
}

fn display(procs: &[Proc]) {
    println!("Found {} processes:", procs.len().to_string().green());
    if !procs.is_empty() {
        let ps = procs
            .iter()
            .enumerate()
            .map(|(i, p)| {
                format!(
                    "{:2}: {:6} {} {}",
                    (i + 1).to_string().yellow(),
                    p.pid.to_string().green(),
                    p.name.blue(),
                    p.args
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n");
        println!("{ps}");
    }
}
