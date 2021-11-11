use std::{iter::repeat, marker::PhantomData};

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
pub struct StatusPage<'a> {
    _life: PhantomData<&'a ()>,
}

impl<'a> StatefulWidget for StatusPage<'a> {
    type State = TuiStates<'a>;
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

        let (up_avg, down_avg) = match state.start_time {
            Some(time) if time.elapsed().as_secs() == 0 => ("?".to_string(), "?".to_string()),
            None => ("?".to_string(), "?".to_string()),
            Some(time) => {
                let elapsed = time.elapsed().as_secs();
                let (up_all, down_all) =
                    state.traffics.iter().fold((0, 0), |(up, down), traffic| {
                        (up + traffic.up, down + traffic.down)
                    });

                (
                    ByteSize(up_all / elapsed).to_string_as(true) + "/s",
                    ByteSize(down_all / elapsed).to_string_as(true),
                )
            }
        };

        let con = &state.connection;
        let con_num = con.connections.len().to_string();
        let height = main[0].height;
        let clash_ver = state
            .version
            .to_owned()
            .map_or_else(|| "?".to_owned(), |v| v.version.to_string());

        let tails = [
            ("Clash Ver.", clash_ver.as_str()),
            ("Clashctl Ver.", crate_version!()),
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
                &(ByteSize(state.max_traffic.up).to_string_as(true) + "/s"),
            ),
            (
                "▼ Max",
                &(ByteSize(state.max_traffic.down).to_string_as(true) + "/s"),
            ),
            ("▲ Total", &ByteSize(con.upload_total).to_string_as(true)),
            ("▼ Total", &ByteSize(con.download_total).to_string_as(true)),
            ("", ""),
        ];

        let info_str = info
            .into_iter()
            .chain(
                repeat(("", ""))
                    .take((height as usize).saturating_sub(info.len() + tails.len() + 2)),
            )
            .chain(tails.into_iter())
            .map(|(title, content)| format!(" {:<15}{:>11} ", title, content))
            .fold(String::with_capacity((30 * height).into()), |mut a, b| {
                a.push_str(&b);
                a.push('\n');
                a
            });

        Paragraph::new(info_str)
            .block(get_block("Info"))
            .style(get_text_style())
            .render(main[0], buf);

        let traffic = Traffics::default();
        traffic.render(main[1], buf, state)
    }
}
