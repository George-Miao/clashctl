use clap::{Parser, Subcommand};
use log::LevelFilter;

use clashctl_interactive::Flags;
use clashctl_tui::TuiOpt;

use crate::{mod_use, utils::init_logger};

mod_use!(completion, proxy, server);

#[derive(Parser, Debug)]
#[clap(
    name = clap::crate_name!(),
    author = clap::crate_authors!(),
    about = clap::crate_description!(),
    license = clap::crate_license!(),
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
