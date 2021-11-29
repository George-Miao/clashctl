use std::borrow::Cow;

use tui::text::{Span, Spans};

pub trait MovableListItem<'a> {
    fn to_spans(&self) -> Spans<'a>;
}

impl<'a> MovableListItem<'a> for Spans<'a> {
    fn to_spans(&self) -> Spans<'a> {
        self.to_owned()
    }
}

impl<'a> MovableListItem<'a> for String {
    fn to_spans(&self) -> Spans<'a> {
        Spans(vec![Span::raw(self.to_owned())])
    }
}

impl<'a> MovableListItem<'a> for Cow<'a, str> {
    fn to_spans(&self) -> Spans<'a> {
        Spans(vec![Span::raw(self.to_owned())])
    }
}

pub trait MovableListItemExt<'a>: MovableListItem<'a> {
    fn width(&self) -> usize {
        self.to_spans().width()
    }
}

impl<'a, T> MovableListItemExt<'a> for T where T: MovableListItem<'a> {}
