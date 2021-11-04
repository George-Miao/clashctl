use clashctl::cli::{init_logger, Cmd, Opts};
use clashctl::Result;

use clap::Parser;
use log::{debug, warn, LevelFilter};

fn main() -> Result<()> {
    let opts = Opts::parse();

    init_logger(match opts.flag.verbose {
        0 => Some(LevelFilter::Info),
        1 => Some(LevelFilter::Debug),
        2 => Some(LevelFilter::Trace),
        _ => None,
    });

    debug!("Opts: {:#?}", opts);

    match opts.cmd {
        Cmd::Completion(arg) => arg.handle(),
        Cmd::Proxy(sub) => sub.handle(&opts.flag),
        Cmd::Server(sub) => sub.handle(&opts.flag),
    }
    .unwrap_or_else(|e| warn!("{}", e));

    Ok(())
}
