use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod completion;
mod proxy;
mod server;

pub use completion::*;
pub use proxy::*;
pub use server::*;

#[derive(Parser, Debug)]
#[clap(
    version = "0.1.0",
    author = "George Miao <gm@miao.dev>",
    about = "CLI used to interact with Clash RESTful API"
)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Cmd,
    #[clap(flatten)]
    pub flag: Flags,
}

#[derive(Parser, Debug)]
pub struct Flags {
    #[clap(
        short,
        long,
        parse(from_occurrences),
        about = "Verbosity. Default: INFO, -v DEBUG, -vv TRACE"
    )]
    pub verbose: u8,
    #[clap(
        short,
        long,
        about = "Timeout of requests, in ms",
        default_value = "2000"
    )]
    pub timeout: u64,
    #[clap(
        short,
        long,
        about = "Path of config file. Default to ~/.config/clashctl/config.ron"
    )]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    #[clap(subcommand)]
    Proxy(ProxySubcommand),
    #[clap(subcommand)]
    Server(ServerSubcommand),
    #[clap(alias = "comp")]
    Completion(CompletionArg),
}
