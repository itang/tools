use std::env;

const FILE_PATH_SEPARATOR: &str = if cfg!(windows) { ";" } else { ":" };

fn main() {
    match env::var("PATH") {
        Ok(s) => {
            let split = s.split(FILE_PATH_SEPARATOR);

            let mut vec: Vec<&str> = split.collect();
            vec.sort_unstable();

            vec.into_iter()
                .filter(|&x| x.trim() != "")
                .enumerate()
                .for_each(|(i, x)| println!("{:3}: {x}", i + 1));
        }
        Err(e) => println!("error: {:?}", e),
    }
}
