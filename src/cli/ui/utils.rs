use std::{
    thread::sleep,
    time::{Duration, Instant},
};

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
