use std::env;

const FILE_PATH_SEPARATOR_CHAR: char = if cfg!(windows) { ';' } else { ':' };

fn main() {
    match env::var("PATH") {
        Ok(s) => {
            let split = s.split(FILE_PATH_SEPARATOR_CHAR);

            let mut vec: Vec<&str> = split.collect();
            vec.sort_unstable();

            vec.into_iter()
                .filter(|&x| !x.trim().is_empty())
                .enumerate()
                .for_each(|(i, x)| println!("{:2}: {x}", i + 1));
        }
        Err(e) => eprintln!("error: {:?}", e),
    }
}
