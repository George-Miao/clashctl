use std::borrow::Cow;

use tui::text::{Span, Spans};

pub trait Wrap: Sized {
    fn wrap_by(self, char: char) -> Self;
    fn wrapped(self) -> Self {
        self.wrap_by(' ')
    }
}

macro_rules! impl_wrap {
    ($t:ty) => {
        impl Wrap for $t {
            fn wrap_by(mut self, char: char) -> Self {
                (&mut self).wrap_by(char);
                self
            }
        }
    };
    ($t:ty, $life:lifetime) => {
        impl<$life> Wrap for $t {
            fn wrap_by(mut self, char: char) -> Self {
                (&mut self).wrap_by(char);
                self
            }
        }
    };
}

impl<'a> Wrap for &mut Span<'a> {
    fn wrap_by(self, char: char) -> Self {
        let content = &mut self.content;
        content.wrap_by(char);
        self
    }
}

impl_wrap!(Span<'a>, 'a);

impl<'a> Wrap for &mut Spans<'a> {
    fn wrap_by(self, char: char) -> Self {
        let inner = &mut self.0;
        match inner.len() {
            0 => {
                inner.push(Span::raw(String::with_capacity(2).wrap_by(char)));
            }
            1 => self.0 = vec![inner[0].to_owned().wrap_by(char)],
            _ => {
                let first = inner.get_mut(0).unwrap();
                first.content = format!("{}{}", char, first.content).into();
                let last = inner.last_mut().unwrap();
                last.content = format!("{}{}", last.content, char).into();
            }
        };
        self
    }
}

impl_wrap!(Spans<'a>, 'a);

impl Wrap for &mut String {
    fn wrap_by(self, char: char) -> Self {
        *self = format!("{}{}{}", char, self, char);
        self
    }
}
impl_wrap!(String);

impl<'a> Wrap for &mut Cow<'a, str> {
    fn wrap_by(self, char: char) -> Self {
        *self = format!("{}{}{}", char, self, char).into();
        self
    }
}

impl_wrap!(Cow<'a, str>, 'a);
