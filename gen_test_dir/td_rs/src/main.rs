use td::gen_dir_str;
use td::DirCreate;

fn main() {
    let dir = gen_dir_str();
    let r = dir.create();
    match r {
        Ok(()) => {
            println!("create {dir} success");
            println!("cd {dir}");
        }
        Err(err) => println!("{err}"),
    }
}
