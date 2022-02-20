use std::env;

const FILE_PATH_SEPARATOR: &str = if cfg!(windows) { ";" } else { ":" };

fn main() {
    match env::var("PATH") {
        Ok(s) => {
            let list = s.split(FILE_PATH_SEPARATOR);

            let mut v: Vec<&str> = list.collect();
            v.sort_unstable();

            v.iter().for_each(|x| println!("{x}"))
        }
        Err(e) => println!("error: {:?}", e),
    }
}
