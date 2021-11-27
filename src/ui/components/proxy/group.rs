use std::{fmt::Debug, marker::PhantomData};

use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
};

use crate::{
    model::ProxyType,
    ui::{
        components::{Consts, ProxyItem},
        utils::get_text_style,
    },
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProxyGroup<'a> {
    pub(super) name: String,
    pub(super) proxy_type: ProxyType,
    pub(super) members: Vec<ProxyItem>,
    pub(super) current: Option<usize>,
    pub(super) cursor: usize,
    pub(super) _life: PhantomData<&'a ()>,
}

pub enum ProxyGroupFocusStatus {
    None,
    Focused,
    Expanded,
}

impl<'a> ProxyGroup<'a> {
    pub fn proxy_type(&self) -> ProxyType {
        self.proxy_type
    }

    pub fn members(&self) -> &Vec<ProxyItem> {
        &self.members
    }
    pub fn get_summary_widget(&self) -> impl Iterator<Item = Span> {
        self.members.iter().map(|x| {
            if x.proxy_type.is_normal() {
                match x.history {
                    Some(ref history) => Self::get_delay_span(history.delay),
                    None => Consts::NO_LATENCY_SPAN,
                }
            } else {
                Consts::NOT_PROXY_SPAN
            }
        })
    }

    pub fn get_widget(&'a self, width: usize, status: ProxyGroupFocusStatus) -> Vec<Spans<'a>> {
        let delimiter = Span::raw(" ");
        let prefix = if matches!(status, ProxyGroupFocusStatus::Focused) {
            Consts::FOCUSED_INDICATOR_SPAN
        } else {
            Consts::UNFOCUSED_INDICATOR_SPAN
        };
        let name = Span::styled(
            &self.name,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

        let proxy_type = Span::styled(self.proxy_type.to_string(), Consts::PROXY_TYPE_STYLE);

        let count = self.members.len();
        let proxy_count = Span::styled(
            if matches!(status, ProxyGroupFocusStatus::Expanded) {
                format!("{}/{}", self.cursor + 1, count)
            } else {
                count.to_string()
            },
            Style::default().fg(Color::Green),
        );

        let mut ret = Vec::with_capacity(if matches!(status, ProxyGroupFocusStatus::Expanded) {
            self.members.len() + 1
        } else {
            2
        });

        ret.push(Spans::from(vec![
            prefix.clone(),
            name,
            delimiter.clone(),
            proxy_type,
            delimiter,
            proxy_count,
        ]));

        if matches!(status, ProxyGroupFocusStatus::Expanded) {
            let skipped = self.cursor.saturating_sub(4);
            let text_style = get_text_style();
            let is_current =
                |index: usize| self.current.map(|x| x == index + skipped).unwrap_or(false);
            let is_pointed = |index: usize| self.cursor == index + skipped;

            let lines = self.members.iter().skip(skipped).enumerate().map(|(i, x)| {
                let prefix = if self.cursor == i + skipped {
                    Consts::EXPANDED_FOCUSED_INDICATOR_SPAN
                } else {
                    Consts::EXPANDED_INDICATOR_SPAN
                };
                let name = Span::styled(
                    &x.name,
                    if is_current(i) {
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::BOLD)
                    } else if is_pointed(i) {
                        text_style.fg(Color::LightBlue)
                    } else {
                        text_style
                    },
                );
                let proxy_type = Span::styled(x.proxy_type.to_string(), Consts::PROXY_TYPE_STYLE);

                let delay_span = x
                    .history
                    .as_ref()
                    .map(|x| {
                        if x.delay > 0 {
                            let style = Self::get_delay_style(x.delay);
                            Span::styled(x.delay.to_string(), style)
                        } else {
                            Span::styled(Consts::NO_LATENCY_SIGN, Consts::NO_LATENCY_STYLE)
                        }
                    })
                    .unwrap_or_else(|| {
                        if !x.proxy_type.is_normal() {
                            Span::raw("")
                        } else {
                            Span::styled(Consts::NO_LATENCY_SIGN, Consts::NO_LATENCY_STYLE)
                        }
                    });
                vec![
                    prefix,
                    Consts::DELIMITER_SPAN.clone(),
                    name,
                    Consts::DELIMITER_SPAN.clone(),
                    proxy_type,
                    Consts::DELIMITER_SPAN.clone(),
                    delay_span,
                ]
                .into()
            });
            ret.extend(lines);
        } else {
            ret.extend(
                self.get_summary_widget()
                    .collect::<Vec<_>>()
                    .chunks(
                        width
                            .saturating_sub(Consts::FOCUSED_INDICATOR_SPAN.width() + 2)
                            .saturating_div(2),
                    )
                    .map(|x| {
                        std::iter::once(if matches!(status, ProxyGroupFocusStatus::Focused) {
                            Consts::FOCUSED_INDICATOR_SPAN
                        } else {
                            Consts::UNFOCUSED_INDICATOR_SPAN
                        })
                        .chain(x.to_owned().into_iter())
                        .collect::<Vec<_>>()
                        .into()
                    }),
            )
        }

        ret
    }

    fn get_delay_style(delay: u64) -> Style {
        match delay {
            0 => Consts::NO_LATENCY_STYLE,
            1..=200 => Consts::LOW_LATENCY_STYLE,
            201..=400 => Consts::MID_LATENCY_STYLE,
            401.. => Consts::HIGH_LATENCY_STYLE,
        }
    }

    fn get_delay_span(delay: u64) -> Span<'static> {
        match delay {
            0 => Consts::NO_LATENCY_SPAN,
            1..=200 => Consts::LOW_LATENCY_SPAN,
            201..=400 => Consts::MID_LATENCY_SPAN,
            401.. => Consts::HIGH_LATENCY_SPAN,
        }
    }
}

impl<'a> Default for ProxyGroup<'a> {
    fn default() -> Self {
        Self {
            members: vec![],
            current: None,
            proxy_type: ProxyType::Selector,
            name: String::new(),
            cursor: 0,
            _life: PhantomData,
        }
    }
}
