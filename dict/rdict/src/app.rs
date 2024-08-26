use std::error::Error;
use std::io;
use std::io::Write;

use ansi_term::Colour;
use clap::Parser;

use rdict::dict;

/// The Opt
#[derive(Parser, Debug)]
#[clap(author, version, name = "rdict", about = "rdict usage.", long_about = None)]
pub struct Args {
    #[clap(help = "Input words")]
    words: Vec<String>,
}

impl Args {
    /// get
    pub fn get() -> Self {
        Args::parse()
    }
}

/// The Router
pub struct Router {
    args: Args,
}

impl Router {
    /// new
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    /// run
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        if self.args.words.is_empty() {
            print!("{}", Colour::Red.paint("Please input the word: "));
            io::stdout().flush()?;
            process_from_input().await;
        } else {
            for word in self.args.words.iter() {
                process_word(word).await;
            }
        }

        Ok(())
    }
}

async fn process_word(word: &str) {
    println!("{}:", Colour::Green.paint(word));

    match dict(word).await {
        Ok(trans) => {
            for p in trans.pronunciation {
                print!("\t {:-6}: {} \t", p.alias, Colour::Blue.paint(&p.pronunciation));
            }
            println!("\n");
            for r in trans.result {
                println!("\t {:-6}: {}", r.part_of_speech, Colour::Blue.paint(&r.explanation));
            }
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
