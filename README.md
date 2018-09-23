# now-cli

[![crates.io version][1]][2]
[![build status][3]][4]
[![docs.rs docs][5]][6]
[![license][7]][8]

> An unofficial Now CLI written in Rust

## Installation
```sh
cargo install now-cli
```

## Usage
```sh
$ now
▲ now 0.1.0
Dan Reeves <hey@danreev.es>
An unofficial Now client written in Rust
USAGE:
    now <SUBCOMMAND>
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    login     Authenticate with Now
    logout    Delete the authentication token
    whoami    Print out who is authenticated
```

## Features

This CLI is in development, to see which features are avaiable see the [TODO file](./TODO.md), or install the CLI and run `now --help`.

## License
[MIT © Dan Reeves](./LICENSE)

[1]: https://img.shields.io/crates/v/now-cli.svg?style=flat-square
[2]: https://crates.io/crates/now-cli
[3]: https://img.shields.io/travis/danreeves/now-cli.svg?style=flat-square
[4]: https://travis-ci.org/danreeves/now-cli
[5]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[6]: https://docs.rs/now-cli
[7]: https://img.shields.io/crates/l/now-cli.svg?style=flat-square
[8]: ./LICENSE
