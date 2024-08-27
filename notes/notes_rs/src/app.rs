use notes::AccountRepository;

use crate::args::{CliArgs, Command};

#[derive(Debug, Clone)]
pub struct Router {
    args: CliArgs,
}

impl Router {
    pub fn new(args: CliArgs) -> Self {
        Self { args }
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let args = self.args;

        match args.command {
            Command::Account(_options) => {
                for account in AccountRepository.list() {
                    println!("{}:{}@{}", account.name, account.mask_password(), account.site)
                }
            },

            Command::Command2(options) => {
                println!("command2 options: {options:?}")
            },
        };

        Ok(())
    }
}
