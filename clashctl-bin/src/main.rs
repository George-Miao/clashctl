use clashctl_cli::{clap::Parser, init_logger, Cmd, Opts};
use clashctl_interactive::{Flags, TuiOpt};
use clashctl_tui::main_loop;
use log::{debug, LevelFilter};

fn main() {
    if std::env::args().len() == 1 {
        let opt = TuiOpt::default();
        let flag = Flags::default();
        if let Err(e) = main_loop(opt, flag) {
            eprintln!("{:?}", e)
        }
    } else {
        let opts = Opts::parse();

        if let Cmd::Tui(opt) = opts.cmd {
            if let Err(e) = main_loop(opt, opts.flag) {
                eprintln!("{:?}", e)
            }
        } else {
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
    }
}
