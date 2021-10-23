use clashctl::cli::{init_logger, Cmd, Opts};
use clashctl::Result;

use clap::Parser;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    init_logger();
    let opts = Opts::parse();

    match opts.cmd {
        Cmd::Completion(arg) => arg.handle()?,
        Cmd::Proxy(sub) => sub.handle().await?,
        Cmd::Server(sub) => sub.handle().await?,
    }
    Ok(())
}
