#![feature(try_blocks)]

#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate rprompt;

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
        _ => unimplemented!(),
    };

    match result {
        Ok(_ok) => {}
        Err(err) => println!("{:?}", err),
    };
}
