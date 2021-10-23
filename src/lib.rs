#[cfg(feature = "cli")]
pub mod cli;

#[cfg(test)]
mod test;

mod api;
mod error;

pub mod model;

pub use api::*;
pub use error::*;
