use std::{env::current_dir, process::ExitCode};

use td::{dir::*, info, warn};

fn main() -> ExitCode {
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

    info!("cd {dir}");

    exit_code
}
