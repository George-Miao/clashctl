use std::time::Instant;

use crate::{
    cli::{Event, InterfaceEvent},
    model::Traffic,
    Result,
};

#[derive(Clone, Debug)]
pub struct TuiStates {
    pub(crate) ticks: u64,
    pub(crate) start_time: Instant,
    pub(crate) traffics: Vec<Traffic>,
    pub(crate) events: Vec<Event>,
    pub(crate) page_index: usize,
}

impl TuiStates {
    pub const TITLES: [&'static str; 4] = ["Status", "Proxies", "Configs", "Debug"];

    pub fn new() -> Self {
        Self {
            ticks: Default::default(),
            start_time: Instant::now(),
            traffics: Default::default(),
            events: Default::default(),
            page_index: Default::default(),
        }
    }

    pub fn handle(&mut self, event: &Event) -> Result<()> {
        match event {
            Event::Interface(InterfaceEvent::TabNext) => self.next(),
            Event::Interface(InterfaceEvent::TabPrev) => self.prev(),
            Event::Interface(InterfaceEvent::TabGoto(index)) => {
                if index >= &1 && index <= &Self::TITLES.len() {
                    self.page_index = *index - 1
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn next(&mut self) {
        self.page_index = (self.page_index + 1) % Self::TITLES.len();
    }

    pub fn prev(&mut self) {
        if self.page_index > 0 {
            self.page_index -= 1;
        } else {
            self.page_index = Self::TITLES.len() - 1;
        }
    }
}

impl Default for TuiStates {
    fn default() -> Self {
        Self::new()
    }
}
