#![feature(generic_associated_types)]

use clashctl::mod_use;
pub use clashctl_core as clashctl;

mod_use![flags, sort, error, config, config_model];
