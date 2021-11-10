use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::model::{Connections, Log, Proxies, Traffic, Version};
use crate::{Error, Result};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Event {
    Quit,
    Interface(InterfaceEvent),
    Update(UpdateEvent),
    Diagnostic(DiagnosticEvent),
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum InterfaceEvent {
    TabNext,
    TabPrev,
    TabGoto(usize),
    ToggleDebug,
    ToggleFocus,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum UpdateEvent {
    Connection(Connections),
    Version(Version),
    Traffic(Traffic),
    Proxies(Proxies),
    Log(Log),
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum DiagnosticEvent {
    Log(String),
}

impl TryFrom<KeyCode> for Event {
    type Error = Error;
    fn try_from(value: KeyCode) -> Result<Self> {
        match value {
            KeyCode::Char('q') | KeyCode::Char('x') => Ok(Event::Quit),
            KeyCode::Char(' ') | KeyCode::Enter => {
                Ok(Event::Interface(InterfaceEvent::ToggleFocus))
            }
            KeyCode::Right => Ok(Event::Interface(InterfaceEvent::TabNext)),
            KeyCode::Left => Ok(Event::Interface(InterfaceEvent::TabPrev)),
            KeyCode::Char(char) => char
                .to_digit(10)
                .ok_or(Error::TuiInternalErr)
                .map(|x| Event::Interface(InterfaceEvent::TabGoto(x as usize))),
            _ => Err(Error::TuiInternalErr),
        }
    }
}

impl TryFrom<KeyEvent> for Event {
    type Error = Error;
    fn try_from(value: KeyEvent) -> Result<Self> {
        match (value.modifiers, value.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c')) => Ok(Self::Quit),
            (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                Ok(Self::Interface(InterfaceEvent::ToggleDebug))
            }
            (KeyModifiers::NONE, key_code) => key_code.try_into(),
            _ => Err(Error::TuiInternalErr),
        }
    }
}
