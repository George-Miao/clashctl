#![feature(generic_associated_types)]

use clashctl::mod_use;

pub use clashctl;

mod_use![config, flags, sort, error,];

#[cfg(feature = "tui")]
mod tui_opt;

#[cfg(feature = "tui")]
pub use tui_opt::*;
