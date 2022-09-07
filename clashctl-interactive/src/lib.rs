#![feature(generic_associated_types)]

pub use clashctl;
use clashctl::mod_use;

mod_use![flags, sort, error, config, config_model];
