use std::{
    collections::{hash_map::DefaultHasher, VecDeque},
    hash::{Hash, Hasher},
    sync::{mpsc::Sender, Mutex},
    thread::sleep,
    time::{Duration, Instant},
};

use log::{LevelFilter, Record};
use tui::style::Color;

use crate::{model, DiagnosticEvent, Event, Result};

pub fn get_hash<T: Hash>(val: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}

pub struct Interval {
    interval: Duration,
    deadline: Option<Instant>,
}

impl Interval {
    pub fn every(interval: Duration) -> Self {
        Self {
            interval,
            deadline: None,
        }
    }

    pub fn next_tick(&mut self) -> Duration {
        let now = Instant::now();
        if self.deadline.is_none() {
            self.deadline = Some(now + self.interval)
        }
        let deadline = self.deadline.unwrap();
        if now > deadline {
            let mut point = deadline;
            loop {
                point += self.interval;
                if point > now {
                    break point - now;
                }
            }
        } else {
            deadline - now
        }
    }

    pub fn tick(&mut self) {
        sleep(self.next_tick())
    }
}

#[test]
fn test_interval() {
    let mut interval = Interval::every(Duration::from_millis(100));
    assert!(interval.next_tick().as_millis().abs_diff(100) < 2);
    sleep(Duration::from_millis(50));
    assert!(interval.next_tick().as_millis().abs_diff(50) < 2);
}

pub struct Pulse {
    pulse: u64,
    counter: u64,
}

impl Pulse {
    #[inline]
    pub fn new(pulse: u64) -> Self {
        Self { pulse, counter: 0 }
    }

    #[inline]
    pub fn tick(&mut self) -> bool {
        let ret = self.is_pulse();
        self.counter += 1;
        ret
    }

    #[inline]
    pub fn is_pulse(&self) -> bool {
        self.counter % self.pulse == 0
    }
}

impl From<model::Level> for Color {
    fn from(val: model::Level) -> Self {
        match val {
            model::Level::Debug => Color::Gray,
            model::Level::Info => Color::Blue,
            model::Level::Warning => Color::Yellow,
            model::Level::Error => Color::Red,
        }
    }
}

pub struct TicksCounter {
    ticks: u64,
    time: Instant,
    inner: VecDeque<u64>,
}

impl Default for TicksCounter {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            ticks: Default::default(),
            inner: Default::default(),
        }
    }
}

impl TicksCounter {
    pub fn new_with_time(time: Instant) -> Self {
        Self {
            time,
            ..Self::default()
        }
    }

    pub fn new_tick(&mut self) {
        self.ticks += 1;
        self.inner.push_front(
            Instant::now()
                .duration_since(self.time)
                .as_millis()
                .try_into()
                .expect(
                    "Hey anyone who sees this as a panic message. Is the universe still there?",
                ),
        );
        if self.inner.len() > 100 {
            self.inner.drain(50..);
        }
    }

    pub fn tick_rate(&self) -> Option<f64> {
        // Ticks per Second
        Some(20_000.0 / ((self.inner.get(0)? - self.inner.get(20)?) as f64))
    }

    pub fn tick_num(&self) -> u64 {
        self.ticks
    }
}

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
        Ok(log::set_boxed_logger(Box::new(self)).map(|()| log::set_max_level(level))?)
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        let text = format!(
            "{:<6}{:?}",
            record.level().to_string().to_uppercase(),
            record.args()
        );
        self.sender
            .lock()
            .unwrap()
            .send(Event::Diagnostic(DiagnosticEvent::Log(text)))
            .unwrap()
    }
    fn flush(&self) {}
}
