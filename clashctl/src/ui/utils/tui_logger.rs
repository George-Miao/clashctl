use std::{fs::File, io::Write, sync::mpsc::Sender};

use log::{LevelFilter, Record};
use simple_mutex::Mutex;

use crate::{ui::TuiResult, DiagnosticEvent, Event};

pub struct LoggerBuilder {
    sender: Sender<Event>,
    file: Option<File>,
    level: LevelFilter,
}

impl LoggerBuilder {
    pub fn new(tx: Sender<Event>) -> Self {
        Self {
            sender: tx,
            file: None,
            level: LevelFilter::Info,
        }
    }

    pub fn file(mut self, file: Option<File>) -> Self {
        self.file = file;
        self
    }

    pub fn level(mut self, level: LevelFilter) -> Self {
        self.level = level;
        self
    }

    pub fn build(self) -> Logger {
        let inner = LoggerInner {
            file: self.file,
            sender: self.sender,
        };
        Logger {
            inner: Mutex::new(inner),
            level: self.level,
        }
    }

    pub fn apply(self) -> TuiResult<()> {
        self.build().apply()
    }
}

struct LoggerInner {
    sender: Sender<Event>,
    file: Option<File>,
}

pub struct Logger {
    inner: Mutex<LoggerInner>,
    level: LevelFilter,
}

impl Logger {
    pub fn apply(self) -> TuiResult<()> {
        let level = self.level;
        Ok(log::set_boxed_logger(Box::new(self)).map(|_| log::set_max_level(level))?)
    }
}

impl log::Log for Logger {
    fn enabled(&self, meta: &log::Metadata) -> bool {
        meta.level() <= self.level
    }

    fn log(&self, record: &Record) {
        let level = record.level();
        let content = record.args().to_string();
        let mut inner = self.inner.lock();
        if let Some(ref mut file) = inner.file {
            writeln!(file, "{:<5} > {}", level, content).unwrap()
        }
        inner
            .sender
            .send(Event::Diagnostic(DiagnosticEvent::Log(level, content))).ok();
            
    }

    fn flush(&self) {}
}
