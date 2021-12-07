#![doc = include_str!("../README.md")]

mod_use![api, error];

#[cfg(test)]
mod test;

pub mod model;

#[cfg(feature = "enum_ext")]
pub use strum;

#[macro_export]
macro_rules! mod_use {
    ($($name:ident $(,)?)+) => {
        $(
            mod $name;
        )+

        $(
            pub use $name::*;
        )+
    };
}
