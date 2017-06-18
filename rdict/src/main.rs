extern crate ansi_term;
extern crate rdict;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

//use std::env;
use std::io;
use ansi_term::Colour;
use structopt::StructOpt;

use rdict::TransResult;

const VERSION: &'static str = "0.2.5";

#[derive(StructOpt, Debug)]
#[structopt(name = "rdict", about = "rdict usage.")]
struct Opt {
    /// A flag, true if used in the command line.
    #[structopt(short = "v", long = "version", help = "show version")]
    version: bool,

    #[structopt(help = "Input word")]
    word: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    if opt.version {
        println!("rdict-v{}", VERSION);
        return;
    }

    if let Some(ref word) = opt.word {
        process_word(word);
    } else {
        loop_process_from_input()
    }
}


//fn word_from_args() -> Option<String> {
//    env::args().nth(1)
//}

fn process_word(word: &str) {
    println!("{}:", Colour::Green.paint(word));

    match rdict::dict(word) {
        Ok(trans) => {
            println!("\t->: {}", Colour::Blue.paint(trans.as_ref()));

            println!("\ntry post to cloud...");
            match rdict::post_to_cloud(&TransResult {
                to: trans,
                from: word.to_string(),
            }) {
                Ok(resp) => println!("\t->: {}", resp),
                Err(err) => println!("error: {}", err),
            }
        }
        Err(err) => println!("\terror: {}", err),
    }
}

fn loop_process_from_input() {
    let mut count = 0;
    loop {
        count += 1;
        println!("[{}]Please input word:", count);
        match word_from_input() {
            Some(ref v) if *v == ":quit" || *v == ":q" => break,
            Some(ref v) => process_word(v),
            None => println!("Please input word."),
        }
        println!("-------------------------------------");
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
