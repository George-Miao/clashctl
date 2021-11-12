#![allow(soft_unstable)]
#![feature(test)]
#![feature(extend_one)]
#![feature(int_abs_diff)]
#![feature(slice_group_by)]
#![feature(thread_is_running)]
#![feature(associated_type_defaults)]

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(test)]
mod test;

mod api;
mod error;

pub mod model;

pub use api::*;
pub use error::*;

#[cfg(feature = "ui")]
mod ui;
#[cfg(feature = "ui")]
pub use ui::TuiOpt;
#[cfg(feature = "ui")]
pub use ui::*;
