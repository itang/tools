#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use std::error::Error;
// use std::fs;
use std::io::{self, Write};
// use std::path::PathBuf;

use ansi_term::Colour;
use clap::Parser;
//use serde::Deserialize;

// use rdict::TransResult;

#[derive(Parser, Debug)]
#[clap(author, version, name = "rdict", about = "rdict usage.", long_about = None)]
struct Opt {
    #[clap(help = "Input words")]
    words: Vec<String>,
}

// #[derive(Debug, Deserialize)]
// struct Config {
//     upstream_url: Option<String>,
// }

// const DICT_LOG_URL: &str = "http://dict.godocking.com/api/dict/logs";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();

    if opt.words.is_empty() {
        print!("{}", Colour::Red.paint("Please input the word: "));
        io::stdout().flush()?;
        process_from_input().await;
    } else {
        for word in opt.words.iter() {
            process_word(word).await;
        }
    }

    Ok(())
}

async fn process_word(word: &str) {
    println!("{}:", Colour::Green.paint(word));

    match rdict::dict(word).await {
        Ok(trans) => {
            println!("\t->: {}", Colour::Blue.paint(trans));
            //post_to_cloud(word, trans)
        },
        Err(err) => println!("\terror: {}", err),
    }
}

async fn process_from_input() {
    match word_from_input() {
        Some(ref v) if *v == ":quit" || *v == ":q" => println!("Bye."),
        Some(ref v) => process_word(v).await,
        None => println!("Error: {}", Colour::Red.paint("Please input word.")),
    }
}

fn word_from_input() -> Option<String> {
    let mut word = String::new();
    let ret = io::stdin().read_line(&mut word);
    if ret.is_ok() && !word.trim().is_empty() {
        return Some(word.trim().to_string());
    }

    None
}

// fn post_to_cloud(from: &str, to: &str) {
//     println!("\ntry post to cloud...");
//
//     let upstream_url = match get_upstream_url_from_config() {
//         Ok(Some(url)) => url,
//         Ok(None) => DICT_LOG_URL.to_string(),
//         Err(e) => {
//             println!("WARN: {}", e);
//             DICT_LOG_URL.to_string()
//         },
//     };
//     println!("post to {}", upstream_url);
//     match rdict::post_to_cloud(&upstream_url, &TransResult { to: to.to_string(), from: from.to_string() }) {
//         Ok(resp) => println!("\t->: {}", resp),
//         Err(err) => println!("error: {}", err),
//     }
// }

// fn get_upstream_url_from_config() -> Result<Option<String>, io::Error> {
//     let mut path: PathBuf = get_app_dir();
//     path.push(".rdict/config");
//     path.set_extension("toml");
//
//     let content = fs::read_to_string(path)?;
//
//     let config: Config = toml::from_str(&content).expect("toml parse error");
//
//     Ok(config.upstream_url)
// }

// fn get_app_dir() -> PathBuf {
//     dirs::home_dir().unwrap_or_else(|| PathBuf::from(""))
// }
