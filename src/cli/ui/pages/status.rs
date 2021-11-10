use bytesize::ByteSize;
use clap::crate_version;
use tui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::cli::components::Traffics;
use crate::cli::{
    components::{get_block, get_text_style},
    TuiStates,
};

#[derive(Clone, Debug, Default)]
pub struct StatusPage {}

impl StatefulWidget for StatusPage {
    type State = TuiStates;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let main = Layout::default()
            .constraints([Constraint::Length(30), Constraint::Min(0)])
            .direction(Direction::Horizontal)
            .split(area);

        let last_traffic = state
            .traffics
            .iter()
            .last()
            .map(|x| x.to_owned())
            .unwrap_or_default();

        let con = &state.connection;
        let info = [
            (
                "▲ Upload",
                ByteSize(last_traffic.up).to_string_as(true) + "/s",
            ),
            (
                "▼ Download",
                ByteSize(last_traffic.down).to_string_as(true) + "/s",
            ),
            (
                "▲ Max",
                ByteSize(state.max_traffic.up).to_string_as(true) + "/s",
            ),
            (
                "▼ Max",
                ByteSize(state.max_traffic.down).to_string_as(true) + "/s",
            ),
            ("▲ Total", ByteSize(con.upload_total).to_string_as(true)),
            ("▼ Total", ByteSize(con.download_total).to_string_as(true)),
            ("Connection #", con.connections.len().to_string()),
            ("Clash Ver.", state.version.version.to_string()),
            ("Clashctl Ver.", crate_version!().to_owned()),
        ]
        .into_iter()
        .map(|(title, content)| format!(" {:<15}{:>11} ", title, content))
        .fold(String::with_capacity(255), |mut a, b| {
            a.push_str(&b);
            a.push('\n');
            a
        });

        Paragraph::new(info)
            .block(get_block("Info"))
            .style(get_text_style())
            .render(main[0], buf);

        let traffic = Traffics::default();
        traffic.render(main[1], buf, state)
    }
}
