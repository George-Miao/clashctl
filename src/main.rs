use clap::Parser;
use clashctl::cli::Opts;
use clashctl::error::Result;

// use clashctl::model::Proxies;
// use serde_json::from_str;

// pub mod lib;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    Ok(())
}
