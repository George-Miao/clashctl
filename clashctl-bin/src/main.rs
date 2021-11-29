use clashctl_interactive::{Flags, TuiOpt};
use clashctl_tui::main_loop;

fn main() {
    if std::env::args().len() == 1 {
        let opt = TuiOpt::default();
        let flag = Flags::default();
        if let Err(e) = main_loop(opt, flag) {
            eprintln!("{:?}", e)
        }
    } else {
        clashctl_cli::run();
    }
}
