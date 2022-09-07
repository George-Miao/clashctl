pub use clap;
use clap::Parser;
use clashctl::mod_use;
pub(crate) use clashctl_interactive::clashctl::{self, model};
use log::{debug, LevelFilter};

use crate::{init_logger, Cmd, Opts};

mod_use![command, proxy_render, utils, error];

pub fn run() {
    let opts = Opts::parse();

    init_logger(match opts.flag.verbose {
        0 => Some(LevelFilter::Info),
        1 => Some(LevelFilter::Debug),
        2 => Some(LevelFilter::Trace),
        _ => None,
    });

    debug!("Opts: {:#?}", opts);

    if let Err(e) = match opts.cmd {
        Cmd::Proxy(sub) => sub.handle(&opts.flag),
        Cmd::Server(sub) => sub.handle(&opts.flag),
        Cmd::Completion(arg) => arg.handle(),
        _ => unreachable!(),
    } {
        eprintln!("{:?}", e)
    }
}
