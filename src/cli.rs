use clap::{Parser, Subcommand};
use clap_generate::Shell;

#[derive(Parser, Debug)]
#[clap(version = "0.0.1", author = "George Miao <gm@miao.dev>")]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Cmd,
    #[clap(flatten)]
    pub config: Config,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    #[clap(subcommand)]
    Proxy(ProxySubcommand),
    Completion(CompletionArg),
}

#[derive(Subcommand, Debug)]
pub enum ProxySubcommand {
    List,
    Select,
}

#[derive(Parser, Debug)]
pub struct CompletionArg {
    #[clap(possible_values=&[
        "bash",
        "elvish",
        "fish",
        "powershell",
        "zsh",
    ])]
    pub shell: Shell,
}

#[derive(Parser, Debug)]
pub struct Config {}
