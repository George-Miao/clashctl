use std::marker::PhantomData;

use tui::widgets::StatefulWidget;

use crate::cli::TuiStates;

#[derive(Clone, Debug, Default)]
pub struct ConfigPage<'a> {
    _life: PhantomData<&'a ()>,
}

impl<'a> StatefulWidget for ConfigPage<'a> {
    type State = TuiStates<'a>;
    fn render(
        self,
        _area: tui::layout::Rect,
        _buf: &mut tui::buffer::Buffer,
        _state: &mut Self::State,
    ) {
    }
}
