use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod completion;
mod proxy;
mod server;

pub use completion::*;
use home::home_dir;
use log::debug;
pub use proxy::*;
pub use server::*;

use crate::cli::Config;
use crate::{Error, Result};

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

impl Flags {
    pub fn get_config(&self) -> Result<Config> {
        let conf_file = self
            .config
            .to_owned()
            .or_else(|| home_dir().map(|dir| dir.join(".config/clashctl/config.ron")))
            .ok_or(Error::ConfigFileOpenError)?;

        if !conf_file.is_file() {
            return Err(Error::ConfigFileTypeError(conf_file));
        }

        if !conf_file.exists() {
            debug!("Config directory does not exist, creating.");
            std::fs::create_dir_all(&conf_file).map_err(Error::ConfigFileIoError)?;
        }
        debug!("Path to config: {}", conf_file.display());
        Config::from_dir(conf_file)
    }
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
