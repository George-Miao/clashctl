use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::model::{Connections, Log, Proxies, Traffic, Version};
use crate::{Error, Result};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Event {
    Quit,
    Interface(Input),
    Update(UpdateEvent),
    Diagnostic(DiagnosticEvent),
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Input {
    Esc,
    TabGoto(usize),
    ToggleDebug,
    ToggleHold,
    List(ListEvent),
    Other(KeyEvent),
}

#[derive(Clone, Debug)]
pub struct ListEvent {
    pub fast: bool,
    pub code: KeyCode,
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
            KeyCode::Esc => Ok(Event::Interface(Input::Esc)),
            KeyCode::Char(' ') | KeyCode::Enter => Ok(Event::Interface(Input::ToggleHold)),
            KeyCode::Char(char) if char.is_ascii_digit() => char
                .to_digit(10)
                .ok_or(Error::TuiInternalErr)
                .map(|x| Event::Interface(Input::TabGoto(x as usize))),
            _ => Err(Error::TuiInternalErr),
        }
    }
}

impl From<KeyEvent> for Event {
    fn from(value: KeyEvent) -> Self {
        match (value.modifiers, value.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c')) => Self::Quit,
            (KeyModifiers::CONTROL, KeyCode::Char('d')) => Self::Interface(Input::ToggleDebug),
            bind @ (modifier, KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down) => {
                Event::Interface(Input::List(ListEvent {
                    fast: matches!(modifier, KeyModifiers::CONTROL | KeyModifiers::SHIFT),
                    code: bind.1,
                }))
            }
            (KeyModifiers::NONE, key_code) => key_code
                .try_into()
                .unwrap_or_else(|_| Self::Interface(Input::Other(value))),
            _ => Self::Interface(Input::Other(value)),
        }
    }
}
