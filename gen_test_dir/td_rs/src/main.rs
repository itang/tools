use td::gen_dir_str;
use td::DirCreate;

fn main() {
    let dir = gen_dir_str();
    let r = dir.create();
    if r {
        println!("create {dir} success");
        println!("cd {dir}");
    } else {
        println!("create {dir} failed");
    }
}
