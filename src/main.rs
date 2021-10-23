use std::io;

use clashctl::cli::{Cmd, Opts};
use clashctl::Result;

use clap::{IntoApp, Parser};
use clap_generate::generate;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        Cmd::Completion(arg) => generate(
            arg.shell,
            &mut Opts::into_app(),
            "clashctl",
            &mut io::stdout(),
        ),
        Cmd::Proxy(sub) => sub.handle().await?,
        Cmd::Server(sub) => sub.handle().await?,
    }
    Ok(())
}
