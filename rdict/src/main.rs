extern crate ansi_term;
extern crate rdict;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::io;
use ansi_term::Colour;
use structopt::StructOpt;

use rdict::TransResult;


#[derive(StructOpt, Debug)]
#[structopt(name = "rdict", about = "rdict usage.")]
struct Opt {
    #[structopt(help = "Input word")]
    word: Option<String>,
}

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
