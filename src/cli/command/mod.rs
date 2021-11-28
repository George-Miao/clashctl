use clap::{Parser, Subcommand};

use crate::{interactive::Flags, mod_use};

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
    pub cmd: Cmd,
    #[clap(flatten)]
    pub flag: Flags,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    #[cfg(feature = "ui")]
    #[clap(about = "Enter tui")]
    Tui(crate::ui::TuiOpt),
    #[clap(subcommand)]
    Proxy(ProxySubcommand),
    #[clap(subcommand)]
    Server(ServerSubcommand),
    #[clap(alias = "comp")]
    Completion(CompletionArg),
}
