use std::iter::repeat;

use bytesize::ByteSize;
use tui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Paragraph, Widget},
};

use crate::ui::{
    components::{MovableListManage, Traffics},
    define_widget, get_block, get_text_style,
};

define_widget!(StatusPage);

impl<'a> Widget for StatusPage<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let main = Layout::default()
            .constraints([Constraint::Length(35), Constraint::Min(0)])
            .direction(Direction::Horizontal)
            .split(area);

        let last_traffic = self
            .state
            .traffics
            .iter()
            .last()
            .map(|x| x.to_owned())
            .unwrap_or_default();

        let (up_avg, down_avg) = match self.state.start_time {
            time if time.elapsed().as_secs() == 0 => ("?".to_string(), "?".to_string()),
            time => {
                let elapsed = time.elapsed().as_secs();
                let (up_all, down_all) = self
                    .state
                    .traffics
                    .iter()
                    .fold((0, 0), |(up, down), traffic| {
                        (up + traffic.up, down + traffic.down)
                    });

                (
                    ByteSize(up_all / elapsed).to_string_as(true) + "/s",
                    ByteSize(down_all / elapsed).to_string_as(true),
                )
            }
        };

        let con_num = self.state.con_state.len().to_string();
        let (total_up, total_down) = self.state.con_size;
        let height = main[0].height;
        let clash_ver = self
            .state
            .version
            .to_owned()
            .map_or_else(|| "?".to_owned(), |v| v.version.to_string());

        let tails = [
            ("Clash Ver.", clash_ver.as_str()),
            ("Clashctl Ver.", env!("CARGO_PKG_VERSION")),
        ];

        let info = [
            ("⇉ Connections", con_num.as_str()),
            (
                "▲ Upload",
                &(ByteSize(last_traffic.up).to_string_as(true) + "/s"),
            ),
            (
                "▼ Download",
                &(ByteSize(last_traffic.down).to_string_as(true) + "/s"),
            ),
            ("▲ Avg.", &up_avg),
            ("▼ Avg.", &down_avg),
            (
                "▲ Max",
                &(ByteSize(self.state.max_traffic.up).to_string_as(true) + "/s"),
            ),
            (
                "▼ Max",
                &(ByteSize(self.state.max_traffic.down).to_string_as(true) + "/s"),
            ),
            ("▲ Total", &ByteSize(total_up).to_string_as(true)),
            ("▼ Total", &ByteSize(total_down).to_string_as(true)),
        ];

        let info_str = info
            .into_iter()
            .chain(
                repeat(("", ""))
                    .take((height as usize).saturating_sub(info.len() + tails.len() + 2)),
            )
            .chain(tails.into_iter())
            .map(|(title, content)| format!(" {:<13}{:>18} ", title, content))
            .fold(String::with_capacity((30 * height).into()), |mut a, b| {
                a.push_str(&b);
                a.push('\n');
                a
            });

        Paragraph::new(info_str)
            .block(get_block("Info"))
            .style(get_text_style())
            .render(main[0], buf);

        let traffic = Traffics::new(self.state);
        traffic.render(main[1], buf)
    }
}
