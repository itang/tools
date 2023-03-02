use std::fs;

use chrono::prelude::*;

pub fn gen_dir_str() -> String {
    let now = Local::now();
    let s = format!("{}", now.format("%m%d"));
    if s.starts_with("0") {
        s[1..].to_string()
    } else {
        s
    }
}

pub trait DirCreate {
    fn create(&self) -> bool;
}

impl DirCreate for String {
    fn create(&self) -> bool {
        matches!(fs::create_dir(self), Ok(()))
    }
}
