use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Paragraph, Widget},
};

use crate::{
    cli::components::{get_block, get_focused_block, get_text_style},
    model::{History, Proxies, Proxy, ProxyType},
};

#[derive(Clone, Debug)]
pub struct ProxyGroup<'a> {
    pub name: String,
    pub proxy_type: ProxyType,
    pub members: Vec<ProxyItem>,
    pub current: Option<usize>,
    _life: PhantomData<&'a ()>,
}

struct Consts {}

impl Consts {
    pub const PROXY_LATENCY_SIGN: &'static str = "⬤ ";

    pub const NOT_PROXY_SIGN: &'static str = "✪ ";

    pub const NO_LATENCY_SIGN: &'static str = "⊝";

    pub const FOCUSED_INDICATOR: &'static str = "🮇  ";

    pub const FOCUSED_EXPANDED_INDICATOR: &'static str = "🮇   ";

    pub const UNFOCUSED_INDICATOR: &'static str = "   ";

    pub const UNFOCUSED_EXPANDED_INDICATOR: &'static str = "    ";

    pub const DEFAULT_STYLE: Style = Style {
        add_modifier: Modifier::empty(),
        sub_modifier: Modifier::empty(),
        fg: None,
        bg: None,
    };

    pub const PROXY_TYPE_STYLE: Style = Style {
        fg: Some(Color::Gray),
        add_modifier: Modifier::DIM,
        ..Self::DEFAULT_STYLE
    };

    pub const NO_LATENCY_STYLE: Style = Style {
        fg: Some(Color::DarkGray),
        ..Self::DEFAULT_STYLE
    };

    pub const LOW_LATENCY_STYLE: Style = Style {
        fg: Some(Color::LightGreen),
        ..Self::DEFAULT_STYLE
    };

    pub const MID_LATENCY_STYLE: Style = Style {
        fg: Some(Color::LightYellow),
        ..Self::DEFAULT_STYLE
    };

    pub const HIGH_LATENCY_STYLE: Style = Style {
        fg: Some(Color::LightRed),
        ..Self::DEFAULT_STYLE
    };

    pub const DELIMITER_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(" "),
        style: Self::DEFAULT_STYLE,
    };

    pub const NOT_PROXY_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::NOT_PROXY_SIGN),
        style: Self::NO_LATENCY_STYLE,
    };

    pub const NO_LATENCY_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::PROXY_LATENCY_SIGN),
        style: Self::NO_LATENCY_STYLE,
    };

    pub const LOW_LATENCY_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::PROXY_LATENCY_SIGN),
        style: Self::LOW_LATENCY_STYLE,
    };

    pub const MID_LATENCY_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::PROXY_LATENCY_SIGN),
        style: Self::MID_LATENCY_STYLE,
    };

    pub const HIGH_LATENCY_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::PROXY_LATENCY_SIGN),
        style: Self::HIGH_LATENCY_STYLE,
    };

    pub const FOCUSED_INDICATOR_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::FOCUSED_INDICATOR),
        style: Style {
            fg: Some(Color::LightYellow),
            ..Self::DEFAULT_STYLE
        },
    };

    pub const FOCUSED_EXPANDED_INDICATOR_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::FOCUSED_EXPANDED_INDICATOR),
        style: Style {
            fg: Some(Color::LightYellow),
            ..Self::DEFAULT_STYLE
        },
    };

    pub const UNFOCUSED_INDICATOR_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::UNFOCUSED_INDICATOR),
        style: Self::DEFAULT_STYLE,
    };

    pub const UNFOCUSED_EXPANDED_INDICATOR_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::UNFOCUSED_EXPANDED_INDICATOR),
        style: Style {
            fg: Some(Color::LightYellow),
            ..Self::DEFAULT_STYLE
        },
    };
}

impl<'a> ProxyGroup<'a> {
    pub(crate) fn get_summary_widget(&self) -> impl Iterator<Item = Span> {
        self.members.iter().map(|x| {
            if x.proxy_type.is_normal() {
                match x.history {
                    Some(History { delay, .. }) => Self::get_delay_span(delay),
                    None => Consts::NO_LATENCY_SPAN,
                }
            } else {
                Consts::NOT_PROXY_SPAN
            }
        })
    }

    pub(crate) fn get_widget(
        &'a self,
        width: usize,
        expanded: bool,
        focused: bool,
    ) -> Vec<Spans<'a>> {
        let delimiter = Span::raw(" ");
        let prefix = if focused {
            Consts::FOCUSED_INDICATOR_SPAN
        } else {
            Consts::UNFOCUSED_INDICATOR_SPAN
        };
        let name = Span::styled(
            self.name.to_owned(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

        let proxy_type = Span::styled(self.proxy_type.to_string(), Consts::PROXY_TYPE_STYLE);

        let proxy_count = Span::styled(
            self.members.len().to_string(),
            Style::default().fg(Color::Green),
        );

        let mut ret = Vec::with_capacity(if expanded { self.members.len() + 1 } else { 2 });

        ret.push(Spans::from(vec![
            prefix.clone(),
            name,
            delimiter.clone(),
            proxy_type,
            delimiter,
            proxy_count,
        ]));

        if expanded {
            let expand_prefix = if focused {
                Consts::FOCUSED_EXPANDED_INDICATOR_SPAN
            } else {
                Consts::UNFOCUSED_EXPANDED_INDICATOR_SPAN
            };

            let text_style = get_text_style();
            let is_current = |index: usize| self.current.map(|x| x == index).unwrap_or(false);

            ret.extend(self.members.iter().enumerate().map(|(i, x)| {
                let name = Span::styled(
                    &x.name,
                    if is_current(i) {
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::BOLD)
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
                    expand_prefix.clone(),
                    Consts::DELIMITER_SPAN.clone(),
                    name,
                    Consts::DELIMITER_SPAN.clone(),
                    proxy_type,
                    Consts::DELIMITER_SPAN.clone(),
                    delay_span,
                ]
                .into()
            }))
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
                        std::iter::once(if focused {
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
            _life: PhantomData,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProxyItem {
    pub name: String,
    pub proxy_type: ProxyType,
    pub history: Option<History>,
    pub udp: bool,
}

impl<'a> From<(&'a str, &'a Proxy)> for ProxyItem {
    fn from(val: (&'a str, &'a Proxy)) -> Self {
        let (name, proxy) = val;
        Self {
            name: name.to_owned(),
            proxy_type: proxy.proxy_type,
            history: proxy.history.get(0).cloned(),
            udp: proxy.udp,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ProxyExpandState {
    pub cursor: usize,
    pub expanded: bool,
}

impl ProxyExpandState {
    pub fn toggle(&mut self) {
        self.expanded = !self.expanded
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProxyTree<'a> {
    pub groups: Vec<ProxyGroup<'a>>,
    pub expand_state: ProxyExpandState,
    pub cursor: usize,
}

impl<'a> From<Proxies> for ProxyTree<'a> {
    fn from(val: Proxies) -> Self {
        let mut ret = Self {
            groups: Vec::with_capacity(val.len()),
            ..Default::default()
        };
        for (name, group) in val.groups() {
            let all = group.all.as_ref().expect("ProxyGroup should have member");
            let mut members = Vec::with_capacity(all.len());
            for x in all.iter() {
                let member = (
                    x.as_str(),
                    val.get(x)
                        .to_owned()
                        .expect("Group member should be in all proxies"),
                )
                    .into();
                members.push(member);
            }

            // if group.now.is_some then it must be in all proxies
            // So use map & expect instead of Option#and_then
            let current = group.now.as_ref().map(|name| {
                members
                    .iter()
                    .position(|item: &ProxyItem| &item.name == name)
                    .expect("Group member should be in all proxies")
            });

            ret.groups.push(ProxyGroup {
                _life: PhantomData,
                name: name.to_owned(),
                proxy_type: group.proxy_type,
                current,
                members,
            })
        }
        ret.groups.sort_by_cached_key(|x| x.name.to_owned());
        ret
    }
}

#[derive(Clone, Debug)]
pub struct ProxyTreeWidget<'a> {
    state: &'a ProxyTree<'a>,

    _life: PhantomData<&'a ()>,
}

impl<'a> ProxyTreeWidget<'a> {
    pub fn new(state: &'a ProxyTree<'a>) -> Self {
        Self {
            _life: PhantomData,
            state,
        }
    }
}

impl<'a> Widget for ProxyTreeWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let cursor = &self.state.cursor;
        let offset = &self.state.expand_state;
        let skip = cursor.saturating_sub(2);
        let text = self
            .state
            .groups
            .iter()
            .skip(skip)
            .enumerate()
            .map(|(i, x)| {
                x.get_widget(
                    area.width as usize,
                    offset.expanded && *cursor == i + skip,
                    i == cursor - skip,
                )
            })
            .reduce(|mut a, b| {
                a.extend(b);
                a
            })
            .unwrap_or_default()
            .into_iter()
            .take(area.height as usize)
            .collect::<Vec<_>>();

        let block = if self.state.expand_state.expanded {
            get_focused_block("Proxies")
        } else {
            get_block("Proxies")
        };

        let mut inner = block.inner(area);
        // if area.height > 12 {
        //     inner.x += 1;
        //     inner.width -= 2;
        //     inner.y += 1;
        //     inner.height -= 2;
        // }
        block.render(area, buf);

        Paragraph::new(text).render(inner, buf);
    }
}
