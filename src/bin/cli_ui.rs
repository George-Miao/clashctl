use clashctl::interactive::Flags;

fn main() {
    use clashctl::{
        cli::{init_logger, Cmd, Opts},
        ui::TuiOpt,
    };

    use clap::Parser;
    use log::{debug, LevelFilter};

    if let Err(e) = if std::env::args().len() == 1 {
        TuiOpt::default().run(Flags::default())
    } else {
        let opts = Opts::parse();

        if let Cmd::Tui(opt) = opts.cmd {
            opt.run(opts.flag)
        } else {
            init_logger(match opts.flag.verbose {
                0 => Some(LevelFilter::Info),
                1 => Some(LevelFilter::Debug),
                2 => Some(LevelFilter::Trace),
                _ => None,
            });

            debug!("Opts: {:#?}", opts);

            match opts.cmd {
                Cmd::Proxy(sub) => sub.handle(&opts.flag),
                Cmd::Server(sub) => sub.handle(&opts.flag),
                Cmd::Completion(arg) => arg.handle(),
                _ => unreachable!(),
            }
        }
    } {
        eprintln!("{:?}", e)
    }
}
