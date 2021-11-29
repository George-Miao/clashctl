use bytesize::ByteSize;
use tui::widgets::Widget;
use tui::{
    layout::{Constraint, Layout},
    symbols::bar::Set,
};
use tui::{
    style::{Color, Style},
    text::Span,
};

use crate::{
    components::{Footer, FooterItem, FooterWidget, Sparkline},
    define_widget,
    utils::get_block,
};

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

pub const REV_DOTS: Set = Set {
    empty: " ",
    one_eighth: "⠁",
    one_quarter: "⠉",
    three_eighths: "⠋",
    half: "⠛",
    five_eighths: "⠟",
    three_quarters: "⠿",
    seven_eighths: "⡿",
    full: "⣿",
};

pub const HALF: Constraint = Constraint::Percentage(50);

define_widget!(Traffics);

impl<'a> Widget for Traffics<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let traffic_size = area.width - 2;

        let traffics = self.state.traffics.iter().rev().take(traffic_size.into());

        let (up, down): (Vec<_>, Vec<_>) = traffics.map(|x| (x.up, x.down)).unzip();

        let (up_max, down_max) = (
            *up.iter().max().unwrap_or(&100),
            *down.iter().max().unwrap_or(&100),
        );

        let title = format!("▲ Max = {}/s", ByteSize(up_max).to_string_as(true));

        let up_line = Sparkline::default()
            .data(&up)
            .max(up_max)
            .bar_set(DOTS)
            .style(Style::default().fg(Color::Green));

        let down_line = Sparkline::default()
            .data(&down)
            .max(down_max)
            .bar_set(REV_DOTS)
            .style(Style::default().fg(Color::White))
            .reversed(true);

        let block = get_block(&title);

        let inner = block.inner(area);

        let layout = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints([HALF, HALF])
            .split(inner);

        block.render(area, buf);
        up_line.render(layout[0], buf);
        down_line.render(layout[1], buf);

        let mut footer = Footer::default();
        footer
            .push_left(FooterItem::span(Span::raw(format!(
                " ▼ Max = {}/s ",
                ByteSize(down_max).to_string_as(true)
            ))))
            .left_offset(1);
        let footer_widget = FooterWidget::new(&footer);
        footer_widget.render(area, buf);
    }
}
