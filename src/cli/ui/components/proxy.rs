use std::marker::PhantomData;

use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Paragraph, Widget},
};

use crate::{
    cli::{
        components::{get_block, get_focused_block},
        Coord,
    },
    model::{Proxies, Proxy, ProxyType},
};

#[derive(Clone, Debug)]
pub struct ProxyGroup<'a> {
    pub name: String,
    pub proxy_type: ProxyType,
    pub members: Vec<ProxyItem>,
    pub current: Option<usize>,
    pub folded: bool,
    _life: PhantomData<&'a ()>,
}

impl<'a> ProxyGroup<'a> {
    pub(crate) fn get_widget(&self) -> Text<'a> {
        let delimiter = Span::raw(" ");

        let name = Span::styled(
            self.name.to_owned(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

        let proxy_type = Span::styled(
            self.proxy_type.to_string(),
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
        );

        let proxy_count = Span::styled(
            self.members.len().to_string(),
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::REVERSED),
        );

        let first_row = Spans::from(vec![
            name,
            delimiter.clone(),
            proxy_type,
            delimiter,
            proxy_count,
        ]);

        Text::from(vec![first_row])
    }
}

impl<'a> Default for ProxyGroup<'a> {
    fn default() -> Self {
        Self {
            members: vec![],
            current: None,
            proxy_type: ProxyType::Selector,
            name: String::new(),
            folded: true,
            _life: PhantomData,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProxyItem {
    pub name: String,
    pub proxy_type: ProxyType,
    pub udp: bool,
}

impl<'a> From<(&'a str, &'a Proxy)> for ProxyItem {
    fn from(val: (&'a str, &'a Proxy)) -> Self {
        Self {
            name: val.0.to_owned(),
            proxy_type: val.1.proxy_type,
            udp: val.1.udp,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProxyTree<'a> {
    pub groups: Vec<ProxyGroup<'a>>,
    pub offset: Coord,
}

impl<'a> From<Proxies> for ProxyTree<'a> {
    fn from(val: Proxies) -> Self {
        let mut ret = Self {
            groups: Vec::with_capacity(val.len()),
            offset: Default::default(),
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
                folded: true,
                proxy_type: group.proxy_type,
                current,
                members,
            })
        }
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
        let text = self
            .state
            .groups
            .iter()
            .map(|x| x.get_widget().lines)
            .reduce(|mut a, b| {
                a.extend(b);
                a
            })
            .unwrap_or_default()
            .into_iter()
            .skip(self.state.offset.y)
            .take(area.height as usize)
            .collect::<Vec<_>>();

        Paragraph::new(text)
            .block(if self.state.offset.hold {
                get_focused_block("Proxies")
            } else {
                get_block("Proxies")
            })
            .render(area, buf);
    }
}
