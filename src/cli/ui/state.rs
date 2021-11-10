use std::time::Instant;

use crate::{
    cli::{Event, InterfaceEvent, UpdateEvent},
    model::{Connections, Log, Proxies, Traffic, Version},
    Result,
};

#[derive(Clone, Debug)]
pub struct TuiStates {
    pub(crate) ticks: u64,
    pub(crate) start_time: Instant,
    pub(crate) traffics: Vec<Traffic>,
    pub(crate) max_traffic: Traffic,
    pub(crate) events: Vec<Event>,
    pub(crate) logs: Vec<Log>,
    pub(crate) page_index: usize,
    pub(crate) connection: Connections,
    pub(crate) version: Version,
    pub(crate) proxies: Proxies,
    pub(crate) show_debug: bool,
    pub(crate) focus: bool,
}

impl TuiStates {
    pub const TITLES: &'static [&'static str] = &["Status", "Proxies", "Logs", "Configs", "Debug"];

    pub fn new() -> Self {
        Self {
            ticks: Default::default(),
            traffics: Default::default(),
            max_traffic: Default::default(),
            events: Default::default(),
            page_index: Default::default(),
            connection: Default::default(),
            proxies: Default::default(),
            logs: Default::default(),
            show_debug: Default::default(),
            focus: Default::default(),

            // Non-default
            start_time: Instant::now(),
            version: Version {
                version: semver::Version::parse("0.0.0").unwrap(),
            },
        }
    }

    pub fn handle(&mut self, event: Event) -> Result<()> {
        self.events.push(event.to_owned());
        match event {
            Event::Interface(event) => self.handle_interface(event),
            Event::Update(update) => self.handle_update(update),
            _ => Ok(()),
        }
    }

    fn handle_update(&mut self, update: UpdateEvent) -> Result<()> {
        match update {
            UpdateEvent::Connection(connection) => self.connection = connection,
            UpdateEvent::Version(version) => self.version = version,
            UpdateEvent::Traffic(traffic) => {
                let Traffic { up, down } = traffic;
                self.max_traffic.up = self.max_traffic.up.max(up);
                self.max_traffic.down = self.max_traffic.down.max(down);
                self.traffics.push(traffic)
            }
            UpdateEvent::Proxies(proxies) => self.proxies = proxies,
            UpdateEvent::Log(log) => self.logs.push(log),
        }
        Ok(())
    }

    fn handle_interface(&mut self, event: InterfaceEvent) -> Result<()> {
        match event {
            InterfaceEvent::TabNext => {
                self.focus = false;
                self.next_page()
            }
            InterfaceEvent::TabPrev => {
                self.focus = false;
                self.prev_page()
            }
            InterfaceEvent::TabGoto(index) => {
                self.focus = false;
                if index >= 1
                    && index <= Self::TITLES.len()
                    && (index - 1 != self.debug_page_index() || self.show_debug)
                {
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
            InterfaceEvent::ToggleFocus => self.focus = !self.focus,
            _ => {}
        }
        Ok(())
    }

    fn next_page(&mut self) {
        self.page_index = (self.page_index + 1) % self.page_len();
    }

    fn prev_page(&mut self) {
        if self.page_index > 0 {
            self.page_index -= 1;
        } else {
            self.page_index = self.page_len() - 1;
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

impl Default for TuiStates {
    fn default() -> Self {
        Self::new()
    }
}
