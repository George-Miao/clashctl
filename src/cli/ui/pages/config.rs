use tui::widgets::StatefulWidget;

use crate::cli::TuiStates;

#[derive(Clone, Debug, Default)]
pub struct ConfigPage {}

impl StatefulWidget for ConfigPage {
    type State = TuiStates;
    fn render(
        self,
        _area: tui::layout::Rect,
        _buf: &mut tui::buffer::Buffer,
        _state: &mut Self::State,
    ) {
    }
}
