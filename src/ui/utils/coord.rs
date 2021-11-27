#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
    pub hold: bool,
}

impl Coord {
    pub fn toggle(&mut self) {
        if self.hold {
            self.end()
        } else {
            self.hold()
        }
    }

    pub fn end(&mut self) {
        *self = Self::default()
    }

    pub fn hold(&mut self) {
        self.hold = true
    }
}
