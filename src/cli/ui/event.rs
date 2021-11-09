use crossterm::event::{KeyCode, KeyEvent};

use crate::model::{Log, Traffic};
use crate::{Error, Result};

#[derive(Clone, Debug)]
pub enum Event {
    Quit,
    TabNext,
    TabPrev,
    TabGoto(u16),
    Traffic(Traffic),
    Log(Log),
    Update,
    Debug(String),
}

pub trait EventHandler {
    fn handle(&mut self, event: &Event) -> Result<()>;
}

impl TryFrom<KeyCode> for Event {
    type Error = Error;
    fn try_from(value: KeyCode) -> Result<Self> {
        match value {
            KeyCode::Char('q') | KeyCode::Char('x') => Ok(Event::Quit),
            KeyCode::Right => Ok(Event::TabNext),
            KeyCode::Left => Ok(Event::TabPrev),
            _ => Err(Error::TuiInternalErr),
        }
    }
}

impl TryFrom<KeyEvent> for Event {
    type Error = Error;
    fn try_from(value: KeyEvent) -> Result<Self> {
        value.code.try_into()
    }
}
