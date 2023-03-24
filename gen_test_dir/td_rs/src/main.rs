use std::{env::current_dir, process::ExitCode};

use colored::Colorize;

use td::gen_dir_str;
use td::DirCreate;

macro_rules! info {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(std::format_args!($($arg)*));
         println!("{}: {}", "INFO".blue(), res.green());
    }}
}

macro_rules! warn {
      ($($arg:tt)*) => {{
        let res = std::fmt::format(std::format_args!($($arg)*));
           println!("{}: {}", "WARN".yellow(), res.red());
    }}
}

fn main() -> ExitCode {
    let dir = gen_dir_str();
    let create_result = dir.create();

    info!("try to create directory: '{dir}' on '{:?}'...", current_dir().expect("get current dir"));

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

    info!("cd {dir}");

    exit_code
}
