use clap::Parser;
use clashctl_interactive::Flags;
use clashctl_tui::{main_loop, TuiOpt};

#[derive(Debug, clap::Parser)]
struct Opt {
    #[clap(flatten)]
    opt: TuiOpt,
    #[clap(flatten)]
    flag: Flags,
}

fn main() {
    let Opt { opt, flag } = Opt::parse();
    if let Err(e) = main_loop(opt, flag) {
        eprintln!("{}", e)
    }
}
