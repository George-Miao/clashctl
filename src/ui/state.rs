use std::{
    collections::VecDeque,
    sync::mpsc::Sender,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use crossterm::event::KeyCode;
use tui::text::Spans;

use crate::{
    model::{Connections, Traffic, Version},
    ui::{
        components::{MovableListState, ProxyTree},
        Event, Input, ListEvent, UpdateEvent,
    },
    Result,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
    pub hold: bool,
}

impl Coord {
    pub fn toggle(&mut self) {
        if self.hold {
            *self = Self::default()
        } else {
            self.hold = true
        }
    }
}

/// # Warn
/// DO NOT USE [`Default::default`] TO INITIALIZE
/// USE [`TuiStates::new`] instead
/// As during runtime we assume all Option field is Some
/// So [`Default`] can be automatically derived2
#[derive(Debug, Default, Clone)]
pub(crate) struct TuiStates<'a> {
    pub(crate) start_time: Option<Instant>,
    pub(crate) version: Option<Version>,
    pub(crate) ticks: u64,
    pub(crate) tick_counter: VecDeque<u64>,
    pub(crate) traffics: Vec<Traffic>,
    pub(crate) max_traffic: Traffic,
    pub(crate) events: Vec<Event>,
    pub(crate) all_events_recv: usize,
    pub(crate) page_index: usize,
    pub(crate) connection: Connections,
    pub(crate) show_debug: bool,
    pub(crate) proxy_tree: ProxyTree<'a>,
    pub(crate) debug_state: MovableListState<'a, String>,
    pub(crate) log_state: MovableListState<'a, Spans<'a>>,
    pub(crate) tx: Option<Sender<Event>>,
}

// TODO impl offset limit in event handling
// Requires MovableListItem to be implemented first
// So content width can be inferred
impl<'a> TuiStates<'a> {
    pub const TITLES: &'static [&'static str] = &["Status", "Proxies", "Logs", "Configs", "Debug"];

    pub fn new(tx: Sender<Event>) -> Self {
        Self {
            start_time: Some(Instant::now()),
            tx: Some(tx),
            ..Default::default()
        }
    }

    pub fn handle(&mut self, event: Event) -> Result<()> {
        self.all_events_recv += 1;
        if self.events.len() >= 300 {
            let _ = self.drop_events(100);
        }
        self.events.push(event.to_owned());
        self.debug_state.items.push(format!("{:?}", event));
        if self.debug_state.offset.hold {
            self.debug_state.offset.y += 1;
        }
        match event {
            Event::Interface(event) => self.handle_input(event),
            Event::Update(update) => self.handle_update(update),
            _ => Ok(()),
        }
    }

    pub fn new_tick(&mut self) {
        self.ticks += 1;
        self.tick_counter.push_front(
            Instant::now()
                .duration_since(self.start_time.unwrap())
                .as_millis()
                .try_into()
                .expect(
                    "Hey anyone who sees this as a panic message. Is the universe still there?",
                ),
        );
        if self.tick_counter.len() > 150 {
            self.tick_counter.drain(100..);
        }
    }

    pub fn page_len(&mut self) -> usize {
        if self.show_debug {
            Self::TITLES.len()
        } else {
            Self::TITLES.len() - 1
        }
    }

    pub fn _get_index(page_name: &str) -> Option<usize> {
        Self::TITLES.iter().position(|x| *x == page_name)
    }

    pub fn title(&self) -> &str {
        Self::TITLES[self.page_index]
    }

    fn handle_update(&mut self, update: UpdateEvent) -> Result<()> {
        match update {
            UpdateEvent::Connection(connection) => self.connection = connection,
            UpdateEvent::Version(version) => self.version = Some(version),
            UpdateEvent::Traffic(traffic) => {
                let Traffic { up, down } = traffic;
                self.max_traffic.up = self.max_traffic.up.max(up);
                self.max_traffic.down = self.max_traffic.down.max(down);
                self.traffics.push(traffic)
            }
            UpdateEvent::Proxies(proxies) => {
                let new_tree = Into::<ProxyTree>::into(proxies);
                self.proxy_tree.merge(new_tree)
            }
            UpdateEvent::Log(log) => {
                self.log_state.items.push(log.into());
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
            Input::ToggleHold => match self.title() {
                "Logs" => self.log_state.offset.toggle(),
                "Debug" => self.debug_state.offset.toggle(),
                "Proxies" => self.proxy_tree.toggle(),
                _ => {}
            },
            Input::List(list_event) => match self.title() {
                "Proxies" => self.handle_proxies_select(list_event),
                _ => self.handle_list(list_event),
            },
            // InterfaceEvent::Other(event) => self.handle_list(event),
            _ => {}
        }
        Ok(())
    }

    fn drop_events(&mut self, num: usize) -> impl Iterator<Item = Event> + '_ {
        self.events.drain(..num)
    }

    fn handle_proxies_select(&mut self, event: ListEvent) {
        let mut tree = &mut self.proxy_tree;
        if tree.expanded {
            let step = if event.fast { 3 } else { 1 };
            let group = &mut tree.groups[tree.cursor];
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
                    if tree.cursor > 0 {
                        tree.cursor = tree.cursor.saturating_sub(1)
                    }
                }
                KeyCode::Down => {
                    if tree.cursor < tree.groups.len() - 1 {
                        tree.cursor = tree.cursor.saturating_add(1)
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_list(&mut self, event: ListEvent) {
        let mut offset = match self.title() {
            "Logs" => &mut self.log_state.offset,
            "Debug" => &mut self.debug_state.offset,
            _ => return,
        };

        if offset.hold {
            match (event.fast, event.code) {
                (true, KeyCode::Left) => offset.x = offset.x.saturating_sub(5),
                (true, KeyCode::Right) => offset.x = offset.x.saturating_add(5),
                (true, KeyCode::Up) => offset.y = offset.y.saturating_sub(5),
                (true, KeyCode::Down) => offset.y = offset.y.saturating_add(5),
                (false, KeyCode::Left) => offset.x = offset.x.saturating_sub(1),
                (false, KeyCode::Right) => offset.x = offset.x.saturating_add(1),
                (false, KeyCode::Up) => offset.y = offset.y.saturating_sub(1),
                (false, KeyCode::Down) => offset.y = offset.y.saturating_add(1),
                _ => {}
            }
        }
    }

    fn debug_page_index(&self) -> usize {
        Self::TITLES.len() - 1
    }
}
