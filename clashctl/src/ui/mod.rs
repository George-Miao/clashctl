pub mod components;
pub mod pages;

mod_use::mod_use![
    utils, action, app, event, servo, state, error, tui_opt, config
];

macro_rules! define_widget {
    ($name:ident) => {
        #[derive(Clone, Debug)]
        pub struct $name<'a> {
            state: &'a $crate::TuiStates<'a>,
            _life: ::std::marker::PhantomData<&'a ()>,
        }

        impl<'a> $name<'a> {
            pub fn new(state: &'a $crate::TuiStates<'a>) -> Self {
                Self {
                    _life: ::std::marker::PhantomData,
                    state,
                }
            }
        }
    };
}

pub(crate) use define_widget;
