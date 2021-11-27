#[macro_export]
macro_rules! define_widget {
    ($name:ident) => {
        #[allow(dead_code)]
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
