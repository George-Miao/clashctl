#![allow(soft_unstable)]
#![feature(test)]
#![feature(extend_one)]
#![feature(int_abs_diff)]
#![feature(slice_group_by)]
#![feature(thread_is_running)]
#![feature(exclusive_range_pattern)]
#![feature(associated_type_defaults)]

#[cfg(test)]
mod test;

mod api;
mod error;

pub mod model;

pub use api::*;
pub use error::*;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "interactive")]
pub mod interactive;

#[cfg(feature = "ui")]
pub mod ui;
