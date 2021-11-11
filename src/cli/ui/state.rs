use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui::text::Spans;

use crate::{
    cli::{components::MovableListState, Event, InterfaceEvent, UpdateEvent},
    model::{Connections, Log, Proxies, Traffic, Version},
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
#[derive(Debug, Default, Clone)]
pub struct TuiStates<'a> {
    pub(crate) start_time: Option<Instant>,
    pub(crate) version: Option<Version>,
    pub(crate) ticks: u64,
    pub(crate) traffics: Vec<Traffic>,
    pub(crate) max_traffic: Traffic,
    pub(crate) events: Vec<Event>,
    pub(crate) all_events_recv: usize,
    pub(crate) page_index: usize,
    pub(crate) connection: Connections,
    pub(crate) proxies: Proxies,
    pub(crate) show_debug: bool,
    pub(crate) debug_state: MovableListState<'a, String>,
    pub(crate) log_state: MovableListState<'a, Spans<'a>>,
}

impl<'a> TuiStates<'a> {
    pub const TITLES: &'static [&'static str] = &["Status", "Proxies", "Logs", "Configs", "Debug"];

    pub fn new() -> Self {
        Self {
            start_time: Some(Instant::now()),
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
            Event::Interface(event) => self.handle_interface(event),
            Event::Update(update) => self.handle_update(update),
            _ => Ok(()),
        }
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
            UpdateEvent::Proxies(proxies) => self.proxies = proxies,
            UpdateEvent::Log(log) => {
                self.log_state.items.push(log.into());
            }
        }
        Ok(())
    }

    fn handle_interface(&mut self, event: InterfaceEvent) -> Result<()> {
        match event {
            InterfaceEvent::TabGoto(index) => {
                self.debug_state.offset = Coord::default();
                self.log_state.offset = Coord::default();
                if index >= 1 && index <= self.page_len() {
                    self.page_index = index - 1
                }
            }
            InterfaceEvent::ToggleDebug => {
                self.show_debug = !self.show_debug;
                // On the debug page
                if self.page_index == Self::TITLES.len() - 1 {
                    self.page_index -= 1;
                } else {
                    self.page_index = self.debug_page_index()
                }
            }
            InterfaceEvent::ToggleHold => self.toggle_hold(),
            InterfaceEvent::Other(event) => self.handle_list(event),
            _ => {}
        }
        Ok(())
    }

    pub fn get_index(page_name: &str) -> Option<usize> {
        Self::TITLES.iter().position(|x| *x == page_name)
    }

    pub fn title(&self) -> &str {
        Self::TITLES[self.page_index]
    }

    pub fn drop_events(&mut self, num: usize) -> impl Iterator<Item = Event> + '_ {
        self.events.drain(..num)
    }

    fn toggle_hold(&mut self) {
        match self.title() {
            "Logs" => self.log_state.offset.toggle(),
            "Debug" => self.debug_state.offset.toggle(),
            _ => {}
        }
    }

    fn handle_list(&mut self, event: KeyEvent) {
        let mut offset = match self.title() {
            "Logs" => &mut self.log_state.offset,
            "Debug" => &mut self.debug_state.offset,
            _ => return,
        };

        if offset.hold && matches!(event.code, KeyCode::Char(' ') | KeyCode::Enter) {
            // No longer holding
            *offset = Coord::default()
        } else if matches!(event.code, KeyCode::Char(' ') | KeyCode::Enter) {
            // Start holding
            offset.hold = true;
        } else if !offset.hold {
            // Other type of input when not holding
        } else {
            // Other type of input when holding
            match (event.modifiers, event.code) {
                (KeyModifiers::SHIFT | KeyModifiers::CONTROL, KeyCode::Left) => {
                    offset.x = offset.x.saturating_sub(5)
                }
                (KeyModifiers::SHIFT | KeyModifiers::CONTROL, KeyCode::Right) => {
                    offset.x = offset.x.saturating_add(5)
                }
                (KeyModifiers::SHIFT | KeyModifiers::CONTROL, KeyCode::Up) => {
                    offset.y = offset.y.saturating_sub(5)
                }
                (KeyModifiers::SHIFT | KeyModifiers::CONTROL, KeyCode::Down) => {
                    offset.y = offset.y.saturating_add(5)
                }
                (_, KeyCode::Left) => offset.x = offset.x.saturating_sub(1),
                (_, KeyCode::Right) => offset.x = offset.x.saturating_add(1),
                (_, KeyCode::Up) => offset.y = offset.y.saturating_sub(1),
                (_, KeyCode::Down) => offset.y = offset.y.saturating_add(1),
                _ => {}
            }
        }
    }

    fn debug_page_index(&self) -> usize {
        Self::TITLES.len() - 1
    }

    pub fn new_tick(&mut self) {
        self.ticks += 1
    }

    pub fn page_len(&mut self) -> usize {
        if self.show_debug {
            Self::TITLES.len()
        } else {
            Self::TITLES.len() - 1
        }
    }
}
