use clashctl::cli::{init_logger, Cmd, Opts};
use clashctl::Result;

use clap::Parser;
use log::{debug, LevelFilter};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    init_logger(match opts.flag.verbose {
        0 => Some(LevelFilter::Info),
        1 => Some(LevelFilter::Debug),
        2 => Some(LevelFilter::Trace),
        _ => None,
    });

    debug!("Opts: {:#?}", opts);

    match opts.cmd {
        Cmd::Completion(arg) => arg.handle()?,
        Cmd::Proxy(sub) => sub.handle(&opts.flag).await?,
        Cmd::Server(sub) => sub.handle(&opts.flag).await?,
    }

    Ok(())
}
