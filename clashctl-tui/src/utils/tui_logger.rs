use std::sync::{mpsc::Sender, Mutex};

use log::{LevelFilter, Record};

use crate::{DiagnosticEvent, Event, Result};

pub struct Logger {
    sender: Mutex<Sender<Event>>,
    level: LevelFilter,
}

impl Logger {
    pub fn new(sender: Sender<Event>) -> Self {
        Self::new_with_level(sender, LevelFilter::Info)
    }

    pub fn new_with_level(sender: Sender<Event>, level: LevelFilter) -> Self {
        Self {
            sender: Mutex::new(sender),
            level,
        }
    }

    pub fn apply(self) -> Result<()> {
        let level = self.level;
        Ok(log::set_boxed_logger(Box::new(self)).map(|_| log::set_max_level(level))?)
    }
}

impl log::Log for Logger {
    fn enabled(&self, meta: &log::Metadata) -> bool {
        meta.level() <= self.level
    }
    fn log(&self, record: &Record) {
        self.sender
            .lock()
            .unwrap()
            .send(Event::Diagnostic(DiagnosticEvent::Log(
                record.level(),
                format!("{}", record.args()),
            )))
            .unwrap()
    }
    fn flush(&self) {}
}
