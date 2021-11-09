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
    pub(crate) events: Vec<Event>,
    pub(crate) logs: Vec<Log>,
    pub(crate) page_index: usize,
    pub(crate) connection: Connections,
    pub(crate) version: Version,
    pub(crate) proxies: Proxies,
}

impl TuiStates {
    pub const TITLES: [&'static str; 4] = ["Status", "Proxies", "Configs", "Debug"];

    pub fn new() -> Self {
        Self {
            ticks: Default::default(),
            traffics: Default::default(),
            events: Default::default(),
            page_index: Default::default(),
            connection: Default::default(),
            proxies: Default::default(),
            logs: Default::default(),

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
            UpdateEvent::Traffic(traffic) => self.traffics.push(traffic),
            UpdateEvent::Proxies(proxies) => self.proxies = proxies,
            UpdateEvent::Log(log) => self.logs.push(log),
        }
        Ok(())
    }

    fn handle_interface(&mut self, event: InterfaceEvent) -> Result<()> {
        match event {
            InterfaceEvent::TabNext => self.next_page(),
            InterfaceEvent::TabPrev => self.prev_page(),
            InterfaceEvent::TabGoto(index) => {
                if index >= 1 && index <= Self::TITLES.len() {
                    self.page_index = index - 1
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn next_page(&mut self) {
        self.page_index = (self.page_index + 1) % Self::TITLES.len();
    }

    fn prev_page(&mut self) {
        if self.page_index > 0 {
            self.page_index -= 1;
        } else {
            self.page_index = Self::TITLES.len() - 1;
        }
    }

    pub fn new_tick(&mut self) {
        self.ticks += 1
    }
}

impl Default for TuiStates {
    fn default() -> Self {
        Self::new()
    }
}
