use clashctl::{interactive::Flags, ui::TuiOpt, Result};

fn main() -> Result<()> {
    TuiOpt::default().run(Flags::default())
}
