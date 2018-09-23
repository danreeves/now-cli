#[derive(StructOpt, Debug)]
#[structopt(
    name = "▲ now",
    about = "Unofficial Now client written in Rust"
)]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "login")]
    /// Log in to your now account
    Login,

    #[structopt(name = "logout")]
    /// Log out of your now account
    Logout,

    #[structopt(name = "whoami")]
    /// Display information about the currently authorized now account
    Whoami,
}
