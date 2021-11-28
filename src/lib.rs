#![allow(soft_unstable)]
#![feature(test)]
#![feature(extend_one)]
#![feature(int_abs_diff)]
#![feature(slice_group_by)]
#![feature(assert_matches)]
#![feature(thread_is_running)]
#![feature(exclusive_range_pattern)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]

mod_use!(api, error);

pub mod model;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "interactive")]
pub mod interactive;

#[cfg(feature = "ui")]
pub mod ui;

#[cfg(test)]
mod test;

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

#[macro_export]
macro_rules! define_widget {
    ($name:ident) => {
        #[derive(Clone, Debug)]
        pub struct $name<'a> {
            state: &'a $crate::ui::TuiStates<'a>,
            _life: ::std::marker::PhantomData<&'a ()>,
        }

        impl<'a> $name<'a> {
            pub fn new(state: &'a $crate::ui::TuiStates<'a>) -> Self {
                Self {
                    _life: ::std::marker::PhantomData,
                    state,
                }
            }
        }
    };
}
