pub use clap;
pub(crate) use clashctl_interactive::clashctl::{self, model};

use clap::Parser;
use clashctl::mod_use;
use clashctl_tui::main_loop;
use log::debug;

use crate::{Cmd, Opts};

mod_use![command, proxy_render, utils, error];

pub fn run() {
    let opts = Opts::parse();
    opts.init_logger();
    debug!("Opts: {:#?}", opts);

    if let Err(e) = match opts.cmd {
        None => main_loop(Default::default(), opts.flag).map_err(Error::TuiError),
        Some(Cmd::Tui(opt)) => main_loop(opt, opts.flag).map_err(Error::TuiError),
        Some(Cmd::Proxy(sub)) => sub.handle(&opts.flag),
        Some(Cmd::Server(sub)) => sub.handle(&opts.flag),
        Some(Cmd::Completion(arg)) => arg.handle(),
    } {
        eprintln!("{}", e)
    }
}
