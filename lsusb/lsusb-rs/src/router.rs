use std::error::Error;

use clap::Parser;

use crate::app::list_devices;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

impl Args {
    pub fn get() -> Self {
        Self::parse()
    }
}

#[derive(Debug, Clone)]
pub struct Router {
    args: Args,
}

impl Router {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        list_devices(self.args.verbose)?;

        Ok(())
    }
}
