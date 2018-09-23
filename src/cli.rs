#[derive(StructOpt, Debug)]
#[structopt(
    name = "▲ now",
    about = "An unofficial Now client written in Rust"
)]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "login")]
    /// Authenticate with Now
    Login,

    #[structopt(name = "logout")]
    /// Delete the authentication token
    Logout,

    #[structopt(name = "whoami")]
    /// Print out who is authenticated
    Whoami,
}
