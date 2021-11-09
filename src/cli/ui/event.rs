use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::model::{Connections, Log, Proxies, Traffic, Version};
use crate::{Error, Result};

#[derive(Clone, Debug)]
pub enum Event {
    Quit,
    Interface(InterfaceEvent),
    Update(UpdateEvent),
    Log(String),
}

#[derive(Clone, Debug)]
pub enum InterfaceEvent {
    TabNext,
    TabPrev,
    TabGoto(usize),
    ToggleDebug,
}

#[derive(Clone, Debug)]
pub enum UpdateEvent {
    Connection(Connections),
    Version(Version),
    Traffic(Traffic),
    Proxies(Proxies),
    Log(Log),
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
        match (value.modifiers, value.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c')) => Ok(Self::Quit),
            (KeyModifiers::CONTROL, KeyCode::Char('d')) => Ok(Self::ToggleDebug),
            (KeyModifiers::NONE, key_code) => key_code.try_into(),
            _ => Err(Error::TuiInternalErr),
        }
    }
}
