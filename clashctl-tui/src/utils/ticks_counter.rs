use std::{collections::VecDeque, time::Instant};

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
