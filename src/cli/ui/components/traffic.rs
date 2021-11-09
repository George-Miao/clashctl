use bytesize::ByteSize;
use tui::style::{Color, Style};
use tui::widgets::{Sparkline, StatefulWidget, Widget};
use tui::{
    layout::{Constraint, Layout},
    symbols::bar::Set,
};

use crate::cli::{components::get_block, TuiStates};

const TRAFFIC_SIZE: usize = 100;

pub const DOTS: Set = Set {
    empty: " ",
    one_eighth: "⡀",
    one_quarter: "⣀",
    three_eighths: "⣄",
    half: "⣤",
    five_eighths: "⣦",
    three_quarters: "⣶",
    seven_eighths: "⣷",
    full: "⣿",
};

#[derive(Default, Clone, Debug)]
pub struct Traffics {}

impl StatefulWidget for Traffics {
    type State = TuiStates;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let half = Constraint::Percentage(50);

        let (mut up, mut down) = ([0; TRAFFIC_SIZE], [0; TRAFFIC_SIZE]);
        for (index, item) in state.traffics.iter().rev().take(TRAFFIC_SIZE).enumerate() {
            up[index] = item.up;
            down[index] = item.down;
        }

        let (up_max, down_max) = (
            *up.iter().max().unwrap_or(&100),
            *down.iter().max().unwrap_or(&100),
        );

        let (up_title, down_title) = (
            format!("Upload (Max = {}/s)", ByteSize(up_max).to_string_as(true)),
            format!(
                "Download (Max = {}/s)",
                ByteSize(down_max).to_string_as(true)
            ),
        );

        let up_line = Sparkline::default()
            .data(&up)
            .max(up_max)
            .bar_set(DOTS)
            .block(get_block(&up_title))
            .style(Style::default().fg(Color::Green));

        let down_line = Sparkline::default()
            .data(&down)
            .max(down_max)
            .bar_set(DOTS)
            .block(get_block(&down_title))
            .style(Style::default().fg(Color::White));

        let layout = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints([half, half])
            .split(area);

        up_line.render(layout[0], buf);
        down_line.render(layout[1], buf);
    }
}
