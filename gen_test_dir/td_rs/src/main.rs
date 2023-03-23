use std::{env::current_dir, process::ExitCode};

use td::gen_dir_str;
use td::DirCreate;

fn main() -> ExitCode {
    let dir = gen_dir_str();
    let r = dir.create();

    println!("try to create directory: '{dir}' on {:?}", current_dir().expect("get current dir"));

    match r {
        Ok(()) => {
            println!("create {dir} success");
            println!("cd {dir}");
            ExitCode::SUCCESS
        },
        Err(err) => {
            println!("{err}");
            ExitCode::FAILURE
        },
    }
}
