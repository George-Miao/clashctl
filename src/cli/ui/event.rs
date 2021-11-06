use crate::model::{Log, Traffic};

pub enum Event {
    Quit,
    Traffic(Traffic),
    Log(Log),
    Update,
}
