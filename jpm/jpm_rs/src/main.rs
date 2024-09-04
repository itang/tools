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
    ///list java process
    List(ListArgs),
    ///kill java process
    Kill(KillArgs),
    ///alias list
    L(ListArgs),
    ///dump java process
    Dump(DumpArgs),
    ///Tips for java process
    Tip(TipArgs),
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
    #[arg(short, long, default_value_t = false)]
    simple: bool,
}

#[derive(Args, Debug)]
struct DumpArgs {
    /// The pid
    #[arg()]
    pid: String,
}

#[derive(Args, Debug)]
struct TipArgs {
    /// The pid
    #[arg()]
    pid: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.action {
        Action::List(args) | Action::L(args) => handle_list(args)?,
        Action::Kill(args) => handle_kill(args)?,
        Action::Dump(args) => handle_dump(args)?,
        Action::Tip(args) => handle_tip(args)?,
    }

    Ok(())
}

fn handle_dump(_args: DumpArgs) -> anyhow::Result<()> {
    todo!()
}

fn handle_tip(args: TipArgs) -> anyhow::Result<()> {
    let pid = args.pid.unwrap_or("xxx".to_string());
    let jts = java_tools(&pid).join("\n");
    println!("{jts}");

    Ok(())
}

fn handle_kill(args: KillArgs) -> anyhow::Result<()> {
    let pid_list: Vec<Proc> = Proc::get_java_process_list(args.glob)?;

    display(&pid_list, true);

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

    display(&pid_list, args.simple);

    Ok(())
}

fn display(procs: &[Proc], simple: bool) {
    println!("Found {} Java processes:", procs.len().to_string().green());
    if !procs.is_empty() {
        let ps = procs
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let jts = if simple {
                    "".to_string()
                } else {
                    java_tools(&p.pid.to_string()).iter().map(|t| format!("\t{t}")).collect::<Vec<String>>().join("\n")
                };
                format!(
                    "{:2}: {:6} {} {}\n{}",
                    (i + 1).to_string().yellow(),
                    p.pid.to_string().green(),
                    p.name.blue(),
                    p.args,
                    jts
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n");
        println!("{ps}");
    } else {
        println!("INFO: No Found Java process, just exit.");
    }
}

fn java_tools(pid: &str) -> Vec<String> {
    let pid = pid.green();
    let jstack = format!("jstack {} | save thread_stack.txt", pid);
    let jmap_heap0 = format!("jmap -heap {}", pid);
    let jmap_heap1 = format!("jhsdb jmap --heap --pid {}", pid);
    let jmap_dump_all0 = format!("jmap -dump:format=b,file=dump.bin {}", pid);
    let jmap_dump_all1 = format!("jmap -dump:all,format=b,file=dump.bin {}", pid);
    let jmap_dump_live = format!("jmap -dump:live,format=b,file=dump.bin {}", pid);
    let jmap_dump_gz_live = format!("jmap -dump:live,gz=9,format=b,file=dump.gz.bin {}", pid);
    let jmap_histo_live = format!("jmap -histo:live {} #触发fullgc", pid);
    let jstat_gc_live = format!("jstat -gc {} 2000", pid);
    let jstat_gcutil_live = format!("jstat -gcutil {} 2000", pid);
    let pidstat = format!("pidstat -w 5 -p {}", pid);
    let kill = format!("kill -f {}", pid);

    vec![
        jstack,
        jmap_heap0,
        jmap_heap1,
        jmap_dump_all0,
        jmap_dump_all1,
        jmap_dump_live,
        jmap_dump_gz_live,
        jmap_histo_live,
        jstat_gc_live,
        jstat_gcutil_live,
        pidstat,
        kill,
    ]
}
