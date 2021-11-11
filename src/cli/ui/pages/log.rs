use std::marker::PhantomData;

use tui::{
    style::Style,
    text::{Span, Spans},
    widgets::StatefulWidget,
};

use crate::{
    cli::{
        components::{GenericStatefulWidget, MovableList},
        TuiStates,
    },
    model::Log,
};

#[derive(Clone, Debug, Default)]
pub struct LogPage<'a> {
    _life: PhantomData<&'a ()>,
}

impl<'a> StatefulWidget for LogPage<'a> {
    type State = TuiStates<'a>;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let list = MovableList::new("Logs");
        GenericStatefulWidget::<Spans>::render(list, area, buf, &mut state.log_state);
    }
}
