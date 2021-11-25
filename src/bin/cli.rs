use clap::Parser;
use log::debug;

use clashctl::{
    cli::{Cmd, Opts},
    Result,
};

fn main() -> Result<()> {
    let opts = Opts::parse();

    opts.flag.init_logger();

    debug!("Opts: {:#?}", opts);

    match opts.cmd {
        Cmd::Proxy(sub) => sub.handle(&opts.flag),
        Cmd::Server(sub) => sub.handle(&opts.flag),
        Cmd::Completion(arg) => arg.handle(),
        _ => unreachable!(),
    }
}
