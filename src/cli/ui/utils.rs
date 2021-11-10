use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use tui::style::Color;

use crate::model;

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
        if now > self.deadline.unwrap() {
            self.interval
        } else {
            self.deadline.unwrap() - now
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
