extern crate ansi_term;
extern crate rdict;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::io;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use ansi_term::Colour;
use structopt::StructOpt;

use rdict::TransResult;


#[derive(StructOpt, Debug)]
#[structopt(name = "rdict", about = "rdict usage.")]
struct Opt {
    #[structopt(help = "Input word")]
    word: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    upstream_url: Option<String>,
}

const DICT_LOG_URL: &'static str = "http://dict.godocking.com/api/dict/logs";

fn main() {
    let opt = Opt::from_args();

    if let Some(ref word) = opt.word {
        process_word(word);
    } else {
        process_from_input()
    }
}

fn process_word(word: &str) {
    println!("{}:", Colour::Green.paint(word));

    match rdict::dict(word) {
        Ok(ref trans) => {
            println!("\t->: {}", Colour::Blue.paint(trans.to_string()));
            post_to_cloud(word, trans)
        }
        Err(err) => println!("\terror: {}", err),
    }
}

fn process_from_input() {
    match word_from_input() {
        Some(ref v) if *v == ":quit" || *v == ":q" => println!("Bye."),
        Some(ref v) => process_word(v),
        None => println!("Error: {}", Colour::Red.paint("Please input word.")),
    }
}

fn word_from_input() -> Option<String> {
    let mut word = String::new();
    let ret = io::stdin().read_line(&mut word);
    if ret.is_ok() && word.trim().len() > 0 {
        return Some(word.trim().to_string());
    }
    return None;
}

fn post_to_cloud(from: &str, to: &str) {
    println!("\ntry post to cloud...");

    let upstream_url = match get_upstream_url_from_config() {
        Ok(Some(url)) => url,
        Ok(None) => DICT_LOG_URL.to_string(),
        Err(e) => {
            println!("WARN: {}", e);
            DICT_LOG_URL.to_string()
        }
    };
    println!("post to {}", upstream_url);
    match rdict::post_to_cloud(
        &upstream_url,
        &TransResult {
            to: to.to_string(),
            from: from.to_string(),
        },
    ) {
        Ok(resp) => println!("\t->: {}", resp),
        Err(err) => println!("error: {}", err),
    }
}

fn get_upstream_url_from_config() -> Result<Option<String>, io::Error> {
    let mut path: PathBuf = get_app_dir();
    path.push(".rdict/config");
    path.set_extension("toml");

    //println!("{:?}", path);
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let config: Config = toml::from_str(&content).expect("toml parse error");

    Ok(config.upstream_url)
}

fn get_app_dir() -> PathBuf {
    let dir: PathBuf = match std::env::home_dir() {
        Some(path) => path,
        None => PathBuf::from(""),
    };
    dir
}
