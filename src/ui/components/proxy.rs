use std::collections::{hash_map::RandomState, HashMap};
use std::{fmt::Debug, marker::PhantomData};

use crossterm::event::KeyCode;
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Paragraph, Widget},
};

use crate::{
    model::{History, Proxies, Proxy, ProxyType},
    ui::{
        components::{Consts, Footer, FooterItem, FooterWidget},
        help_footer,
        utils::{get_block, get_focused_block, get_text_style},
        ListEvent,
    },
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProxyGroup<'a> {
    pub name: String,
    pub proxy_type: ProxyType,
    pub members: Vec<ProxyItem>,
    pub current: Option<usize>,
    pub cursor: usize,
    _life: PhantomData<&'a ()>,
}

pub enum ProxyGroupFocusStatus {
    None,
    Focused,
    Expanded,
}

impl<'a> ProxyGroup<'a> {
    pub fn get_summary_widget(&self) -> impl Iterator<Item = Span> {
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProxyItem {
    pub name: String,
    pub proxy_type: ProxyType,
    pub history: Option<History>,
    pub udp: bool,
    pub now: Option<String>,
}

impl<'a> From<(&'a str, &'a Proxy)> for ProxyItem {
    fn from(val: (&'a str, &'a Proxy)) -> Self {
        let (name, proxy) = val;
        Self {
            name: name.to_owned(),
            proxy_type: proxy.proxy_type,
            history: proxy.history.get(0).cloned(),
            udp: proxy.udp,
            now: proxy.now.as_ref().map(Into::into),
        }
    }
}

// TODO Proxy tree furthur functions
//
// - Right & Enter can be used to apply selection
// - Esc for exist expand mode
// - T for test latency of current group
// - S for switch between sorting strategies
// - / for searching
//
// In order for functions to be implemented, these are required:
// - Remove Enter from InterfaceEvent::ToggleHold
// - Maybe a new InterfaceEvent::Confirm correstponds to Enter
// - `T`, `S`, `/` in proxy event handling
#[derive(Clone, Debug, PartialEq)]
pub struct ProxyTree<'a> {
    groups: Vec<ProxyGroup<'a>>,
    expanded: bool,
    cursor: usize,
    testing: bool,
    footer: Footer<'a>,
}

impl<'a> Default for ProxyTree<'a> {
    fn default() -> Self {
        let mut ret = Self {
            groups: Default::default(),
            expanded: Default::default(),
            cursor: Default::default(),
            footer: Default::default(),
            testing: Default::default(),
        };
        ret.update_footer();
        ret
    }
}

impl<'a> ProxyTree<'a> {
    pub fn toggle(&mut self) {
        self.expanded = !self.expanded;
        self.update_footer();
    }

    pub fn end(&mut self) {
        self.expanded = false;
        self.update_footer();
    }

    pub fn handle(&mut self, event: ListEvent) {
        if self.expanded {
            let step = if event.fast { 3 } else { 1 };
            let group = &mut self.groups[self.cursor];
            match event.code {
                KeyCode::Up => {
                    if group.cursor > 0 {
                        group.cursor = group.cursor.saturating_sub(step)
                    }
                }
                KeyCode::Down => {
                    let left = group.members.len().saturating_sub(group.cursor + 1);
                    if left > 0 {
                        group.cursor += left.min(step)
                    }
                }
                _ => {}
            }
        } else {
            match event.code {
                KeyCode::Up => {
                    if self.cursor > 0 {
                        self.cursor = self.cursor.saturating_sub(1)
                    }
                }
                KeyCode::Down => {
                    if self.cursor < self.groups.len() - 1 {
                        self.cursor = self.cursor.saturating_add(1)
                    }
                }
                _ => {}
            }
        }
        self.update_footer()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.groups.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    #[inline]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    #[inline]
    pub fn current_group(&self) -> &ProxyGroup {
        &self.groups[self.cursor]
    }

    #[inline]
    pub fn testing(&self) -> bool {
        self.testing
    }

    #[inline]
    pub fn start_testing(&mut self) {
        self.testing = true;
        self.update_footer()
    }

    #[inline]
    pub fn end_testing(&mut self) {
        self.testing = false;
        self.update_footer()
    }

    pub fn update_footer(&mut self) {
        let mut footer = Footer::default();
        let current_group = match self.groups.get(self.cursor) {
            Some(grp) => grp,
            _ => return,
        };

        if !self.expanded {
            let group_name = current_group.name.clone();
            let style = Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::REVERSED);
            let highlight = style.add_modifier(Modifier::BOLD);

            let mut left = vec![
                FooterItem::span(Span::styled(" FREE ", style)),
                FooterItem::span(Span::styled(" SPACE to expand ", style)),
                if !self.testing {
                    FooterItem::spans(help_footer("Test", style, highlight)).wrapped()
                } else {
                    FooterItem::span(Span::styled(" Testing ", highlight.fg(Color::Green)))
                },
                FooterItem::spans(help_footer("Sort", style, highlight)).wrapped(),
            ];

            footer.append_left(&mut left);

            let name = FooterItem::span(Span::styled(group_name, style)).wrapped();
            footer.push_right(name);

            if let Some(now) = current_group.current {
                footer.push_right(
                    FooterItem::span(Span::raw(current_group.members[now].name.to_owned()))
                        .wrapped(),
                );
            }
        } else {
            let style = Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::REVERSED);
            let highlight = style.add_modifier(Modifier::BOLD);

            footer.push_left(FooterItem::span(Span::styled(" [^] ▲ ▼ Move ", style)));

            if current_group.proxy_type.is_selector() {
                footer.push_left(FooterItem::span(Span::styled(" ▶ Select ", style)));
            }

            let current_item = &current_group.members[current_group.cursor];
            footer.push_left(if !self.testing {
                FooterItem::spans(help_footer("Test", style, highlight)).wrapped()
            } else {
                FooterItem::span(Span::styled(" Testing ", highlight.fg(Color::Blue)))
            });

            footer.push_left(FooterItem::spans(help_footer("Sort", style, highlight)).wrapped());

            if let Some(now) = &current_item.now {
                footer.push_right(FooterItem::span(Span::raw(now.to_owned())).wrapped());
            }
        }
        self.footer = footer
    }

    pub fn sync_cursor_from(&mut self, mut new: ProxyTree<'a>) {
        if self == &new {
            return;
        }
        new.expanded = self.expanded;
        let map = HashMap::<_, _, RandomState>::from_iter(
            self.groups.iter().map(|x| (x.name.to_owned(), x)),
        );
        let current_group = self.groups.get(self.cursor);
        for (index, new_group) in new.groups.iter_mut().enumerate() {
            if let Some(true) = current_group.map(|x| x.name == new_group.name) {
                new.cursor = index;
            }
            if let Some(old_group) = map.get(&new_group.name) {
                new_group.cursor = old_group
                    .members
                    .get(old_group.cursor)
                    .and_then(|old_member| {
                        new_group
                            .members
                            .iter()
                            .position(|new_member| new_member.name == old_member.name)
                    })
                    .or(new_group.current)
                    .unwrap_or_default()
            }
        }
        *self = new;
        self.update_footer()
    }
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
                cursor: current.unwrap_or_default(),
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
}

impl<'a> ProxyTreeWidget<'a> {
    pub fn new(state: &'a ProxyTree<'a>) -> Self {
        Self { state }
    }
}

impl<'a> Widget for ProxyTreeWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let cursor = &self.state.cursor;
        let skip = if self.state.expanded {
            *cursor
        } else {
            cursor.saturating_sub(2)
        };
        let text = self
            .state
            .groups
            .iter()
            .skip(skip)
            .enumerate()
            .map(|(i, x)| {
                x.get_widget(
                    area.width as usize,
                    match (self.state.expanded, *cursor == i + skip) {
                        (true, true) => ProxyGroupFocusStatus::Expanded,
                        (false, true) => ProxyGroupFocusStatus::Focused,
                        _ => ProxyGroupFocusStatus::None,
                    },
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

        let block = if self.state.expanded {
            get_focused_block("Proxies")
        } else {
            get_block("Proxies")
        };

        let inner = block.inner(area);

        block.render(area, buf);

        Paragraph::new(text).render(inner, buf);
        FooterWidget::new(&self.state.footer).render(area, buf);
    }
}
