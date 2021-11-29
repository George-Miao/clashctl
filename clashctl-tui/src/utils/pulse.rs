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
