use notes::AccountRepository;

use crate::args::{AccountOptions, CliArgs, Command};

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
            Some(Command::Account(_options)) => handle_account_command(_options),
            None => handle_account_command(AccountOptions { all: false, token: None }),
        };

        Ok(())
    }
}

fn handle_account_command(account_options: AccountOptions) {
    println!("Accounts:");
    for (index, account) in AccountRepository.list().iter().enumerate() {
        if (account_options.all && account_options.token == Some("yolo".into())) || account.pubilc {
            println!("{:-2} - {}:{}@{}", index + 1, account.name, account.mask_password(), account.site)
        }
    }
    println!();
}
