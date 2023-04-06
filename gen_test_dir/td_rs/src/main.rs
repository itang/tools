//! td-cli main.
#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::{env::current_dir, process::ExitCode};

use clap::Parser;
use serde::Serialize;

use td::{gen_dir_str, info, warn, DirCreate};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output to json format
    #[arg(long, default_value_t = false)]
    json: bool,
}

#[derive(Serialize, Debug)]
struct Data {
    dir: String,
}

fn main() -> ExitCode {
    let args = Args::parse();
    td::SILENT_MODE.set(args.json).expect("set value");

    let dir = gen_dir_str();
    let create_result = dir.create();

    info!("try to create directory: '{dir}' on '{}'...", current_dir().expect("get current dir").as_path().display());

    let exit_code = match create_result {
        Ok(()) => {
            info!("create '{dir}' directory success");
            ExitCode::SUCCESS
        },
        Err(err) => {
            warn!("{err}");
            ExitCode::FAILURE
        },
    };

    if args.json {
        let data = Data { dir: dir.clone() };
        let json = serde_json::to_string_pretty(&data).expect("to json");
        println!("{json}");
    }

    info!("cd {dir}");

    exit_code
}
