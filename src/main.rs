#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate tabular;
extern crate chrono;
extern crate chrono_humanize;
extern crate console;
extern crate dialoguer;
extern crate directories;
extern crate mkdirp;
extern crate reqwest;
extern crate rprompt;
extern crate serde_json;

#[allow(non_camel_case_types)]
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
