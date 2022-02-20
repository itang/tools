use anyhow::Result;
use std::env;
use tap::prelude::*;

const FILE_PATH_SEPARATOR_CHAR: char = if cfg!(windows) { ';' } else { ':' };
const DEFAULT_ENV_NAME: &str = "PATH";

fn main() -> Result<()> {
    /*
    let name = env::args().nth(1);
    let name = name
        .as_ref()
        .map(String::as_ref)
        .unwrap_or(DEFAULT_ENV_NAME);
    */

    let name = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_ENV_NAME.to_owned());

    name.pipe(env::var)?
        .split(FILE_PATH_SEPARATOR_CHAR)
        .collect::<Vec<&str>>()
        .tap_mut(|v| v.sort_unstable())
        .into_iter()
        .filter(|&x| !x.trim().is_empty())
        .enumerate()
        .for_each(|(i, x)| println!("{:2}: {x}", i + 1));

    Ok(())
}
