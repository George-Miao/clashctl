use std::time::Instant;

use tui::{layout::Rect, Frame};

use crate::{
    components::MovableListItem,
    model::{Rules, Traffic, Version},
    ui::{
        components::{MovableListState, ProxyTree},
        pages::{
            ConfigPage, ConnectionsPage, DebugPage, LogPage, ProxiesPage, RulesPage, StatusPage,
        },
        Event, Input, UpdateEvent,
    },
    Backend, Result,
};

/// # Warn
/// DO NOT USE [`Default::default`] TO INITIALIZE
///
/// USE [`TuiStates::new`] instead
///
/// As during runtime we assume all Option field is Some.
/// So [`Default`] can be automatically derived2
#[derive(Debug, Default, Clone)]
pub struct TuiStates<'a> {
    pub(crate) should_quit: bool,
    pub(crate) start_time: Option<Instant>,
    pub(crate) version: Option<Version>,
    pub(crate) traffics: Vec<Traffic>,
    pub(crate) max_traffic: Traffic,
    pub(crate) events: Vec<Event>,
    pub(crate) all_events_recv: usize,
    pub(crate) page_index: usize,
    pub(crate) show_debug: bool,
    pub(crate) proxy_tree: ProxyTree<'a>,
    pub(crate) debug_state: MovableListState<'a>,
    pub(crate) log_state: MovableListState<'a>,
    pub(crate) con_state: MovableListState<'a>,
    pub(crate) rule_state: MovableListState<'a>,
    pub(crate) con_size: (u64, u64),
    pub(crate) rules: Rules,
    // pub(crate) tx: Option<Sender<Event>>,
}

impl<'a> TuiStates<'a> {
    pub const TITLES: &'static [&'static str] = &[
        "Status", "Proxies", "Rules", "Conns", "Logs", "Configs", "Debug",
    ];

    #[inline]
    pub fn new() -> Self {
        Self {
            start_time: Some(Instant::now()),
            // tx: Some(tx),
            ..Default::default()
        }
    }

    pub fn handle(&mut self, event: Event) -> Result<()> {
        self.all_events_recv += 1;
        if self.events.len() >= 300 {
            let _ = self.drop_events(100);
        }
        self.events.push(event.to_owned());
        self.debug_state
            .push(MovableListItem::Spans((&event).into()));

        match event {
            Event::Quit => {
                self.should_quit = true;
                Ok(())
            }
            Event::Interface(event) => self.handle_input(event),
            Event::Update(update) => self.handle_update(update),
            _ => Ok(()),
        }
    }

    #[inline]
    pub fn page_len(&mut self) -> usize {
        if self.show_debug {
            Self::TITLES.len()
        } else {
            Self::TITLES.len() - 1
        }
    }

    #[inline]
    pub fn title(&self) -> &str {
        Self::TITLES[self.page_index]
    }

    pub fn active_list_state(&mut self) -> Option<&mut MovableListState<'a>> {
        match self.title() {
            "Logs" => Some(&mut self.log_state),
            "Debug" => Some(&mut self.debug_state),
            "Rules" => Some(&mut self.rule_state),
            "Conns" => Some(&mut self.con_state),
            _ => None,
        }
    }

    pub fn render_route(&self, area: Rect, f: &mut Frame<Backend>) {
        match self.page_index {
            0 => f.render_widget(StatusPage::new(self), area),
            1 => f.render_widget(ProxiesPage::new(self), area),
            2 => f.render_widget(RulesPage::new(self), area),
            3 => f.render_widget(ConnectionsPage::new(self), area),
            4 => f.render_widget(LogPage::new(self), area),
            5 => f.render_widget(ConfigPage::new(self), area),
            6 => f.render_widget(DebugPage::new(self), area),
            _ => unreachable!(),
        };
    }

    fn handle_update(&mut self, update: UpdateEvent) -> Result<()> {
        match update {
            UpdateEvent::Connection(connection) => {
                self.con_size = (connection.upload_total, connection.download_total);
                self.con_state.merge(connection.into());
            }
            UpdateEvent::Version(version) => self.version = Some(version),
            UpdateEvent::Traffic(traffic) => {
                let Traffic { up, down } = traffic;
                self.max_traffic.up = self.max_traffic.up.max(up);
                self.max_traffic.down = self.max_traffic.down.max(down);
                self.traffics.push(traffic)
            }
            UpdateEvent::Proxies(proxies) => {
                let new_tree = Into::<ProxyTree>::into(proxies);
                self.proxy_tree.sync_cursor_from(new_tree);
            }
            UpdateEvent::Log(log) => {
                self.log_state.push(MovableListItem::Spans(log.into()));
            }
            UpdateEvent::Rules(rules) => {
                self.rules = rules;
            }
        }
        Ok(())
    }

    fn handle_input(&mut self, event: Input) -> Result<()> {
        match event {
            Input::TabGoto(index) => {
                if index >= 1 && index <= self.page_len() {
                    self.page_index = index - 1
                }
            }
            Input::ToggleDebug => {
                self.show_debug = !self.show_debug;
                // On the debug page
                if self.page_index == Self::TITLES.len() - 1 {
                    self.page_index -= 1;
                } else if self.show_debug {
                    self.page_index = self.debug_page_index()
                }
            }
            Input::ToggleHold => match self.active_list_state() {
                Some(state) => state.toggle(),
                None => {
                    if self.title() == "Proxies" {
                        self.proxy_tree.toggle()
                    }
                }
            },
            Input::List(event) => match self.active_list_state() {
                Some(state) => state.handle(event),
                None => {
                    if self.title() == "Proxies" {
                        self.proxy_tree.handle(event)
                    }
                }
            },
            // InterfaceEvent::Other(event) => self.handle_list(event),
            _ => {}
        }
        Ok(())
    }

    pub fn debug_page_index(&self) -> usize {
        Self::TITLES.len() - 1
    }

    fn drop_events(&mut self, num: usize) -> impl Iterator<Item = Event> + '_ {
        self.events.drain(..num)
    }
}
