use std::{cmp::Ordering, collections::HashMap};
use std::{fmt::Debug, marker::PhantomData};

use crossterm::event::KeyCode;
use tui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::{
    model::Proxies,
    ui::{
        components::{Footer, FooterItem, ProxyGroup, ProxyItem},
        help_footer, Action, ListEvent,
    },
};

// TODO Proxy tree furthur functions
//
// - [X] Right & Enter can be used to apply selection
// - [X] Esc for exist expand mode
// - [X] T for test latency of current group
// - [ ] S for switch between sorting strategies
// - [ ] / for searching
//
// In order for functions to be implemented, these are required:
// - Remove Enter from InterfaceEvent::ToggleHold
// - Maybe a new InterfaceEvent::Confirm correstponds to Enter
// - `T`, `S`, `/` in proxy event handling
#[derive(Clone, Debug, PartialEq)]
pub struct ProxyTree<'a> {
    pub(super) groups: Vec<ProxyGroup<'a>>,
    pub(super) expanded: bool,
    pub(super) cursor: usize,
    pub(super) testing: bool,
    pub(super) footer: Footer<'a>,
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
    #[inline]
    pub fn toggle(&mut self) -> &mut Self {
        self.expanded = !self.expanded;
        self.update_footer()
    }

    #[inline]
    pub fn end(&mut self) -> &mut Self {
        self.expanded = false;
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
    pub fn is_testing(&self) -> bool {
        self.testing
    }

    #[inline]
    pub fn start_testing(&mut self) -> &mut Self {
        self.testing = true;
        self.update_footer()
    }

    #[inline]
    pub fn end_testing(&mut self) -> &mut Self {
        self.testing = false;
        self.update_footer()
    }

    pub fn sort_with_frequency(&mut self, freq: &HashMap<String, usize>) -> &mut Self {
        self.groups
            .sort_by(|a, b| match (freq.get(&a.name), freq.get(&b.name)) {
                (Some(a_freq), Some(b_freq)) => b_freq.cmp(a_freq),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => a.name.cmp(&b.name),
            });
        self
    }

    pub fn handle(&mut self, event: ListEvent) -> Option<Action> {
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

                    group.cursor += left.min(step)
                }
                KeyCode::Right | KeyCode::Enter => {
                    if group.proxy_type.is_selector() {
                        let current = group.members[group.cursor].name.to_owned();
                        return Some(Action::ApplySelection {
                            group: group.name.to_owned(),
                            proxy: current,
                        });
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
                KeyCode::Enter => self.expanded = true,
                _ => {}
            }
        }
        self.update_footer();
        None
    }

    pub fn update_footer(&mut self) -> &mut Self {
        let mut footer = Footer::default();
        let current_group = match self.groups.get(self.cursor) {
            Some(grp) => grp,
            _ => return self,
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
        self.footer = footer;
        self
    }

    pub fn replace_with(&mut self, mut new_tree: ProxyTree<'a>) -> &mut Self {
        if self == &new_tree {
            return self;
        }
        new_tree.expanded = self.expanded;
        // let map = HashMap::<_, _, RandomState>::from_iter(self.groups.iter().map(|x| (&x.name, x)));
        let old_groups = &self.groups;
        let current_group = self.groups.get(self.cursor);
        for (index, new_group) in new_tree.groups.iter_mut().enumerate() {
            if let Some(true) = current_group.map(|x| x.name == new_group.name) {
                new_tree.cursor = index;
            }
            if let Some(old_group) = old_groups.iter().find(|group| group.name == new_group.name) {
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
        *self = new_tree;
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
            let all = group
                .all
                .as_ref()
                .expect("ProxyGroup should have member vec");
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

        ret
    }
}
