use clap::{Parser, Subcommand};
use log::LevelFilter;

use crate::{interactive::Flags, ui::TuiOpt, utils::init_logger};

mod_use::mod_use!(completion, proxy, server);

#[derive(Parser, Debug)]
#[clap(
    name = clap::crate_name!(),
    author = clap::crate_authors!(),
    about = clap::crate_description!(),
    version = clap::crate_version!(),

)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Option<Cmd>,
    #[clap(flatten)]
    pub flag: Flags,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    #[clap(about = "Open TUI")]
    Tui(TuiOpt),
    #[clap(subcommand)]
    Proxy(ProxySubcommand),
    #[clap(subcommand)]
    Server(ServerSubcommand),
    #[clap(alias = "comp")]
    Completion(CompletionArg),
}

impl Opts {
    pub fn init_logger(&self) {
        if matches!(self.cmd, Some(Cmd::Tui(_)) | None) {
            return;
        }
        init_logger(match self.flag.verbose {
            0 => Some(LevelFilter::Info),
            1 => Some(LevelFilter::Debug),
            2 => Some(LevelFilter::Trace),
            _ => None,
        });
    }
}
