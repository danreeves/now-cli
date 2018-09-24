#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate tabular;
extern crate rprompt;

mod api;
mod cli;
mod handlers;

use self::cli::{Cli, Command};
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    let result = match args.command {
        Command::Login => handlers::login(),
        Command::Logout => handlers::logout(),
        Command::Whoami => handlers::whoami(),
        Command::List => handlers::list(),
        _ => unimplemented!(),
    };

    match result {
        Ok(_ok) => {}
        Err(err) => println!("{:?}", err),
    };
}
