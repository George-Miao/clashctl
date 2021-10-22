use std::io;

use clashctl::cli::{Cmd, Opts, ProxySubcommand};
use clashctl::Result;

use clap::{IntoApp, Parser};
use clap_generate::generate;

// use clashctl::model::Proxies;
// use serde_json::from_str;

// pub mod lib;

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
        Cmd::Proxy(sub) => match sub {
            ProxySubcommand::List => {}
            ProxySubcommand::Select => {}
        },
    }
    Ok(())
}
