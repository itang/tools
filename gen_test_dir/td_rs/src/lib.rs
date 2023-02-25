use chrono::prelude::*;
use std::fs;

pub fn gen_dir_str() -> String {
    let now = Local::now();
    format!("{}", now.format("%d_%m"))
}

pub trait DirCreate {
    fn create(&self) -> bool;
}

impl DirCreate for String {
    fn create(&self) -> bool {
        matches!(fs::create_dir(self), Ok(()))
    }
}
