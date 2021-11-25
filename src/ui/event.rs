use std::fmt::Display;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::Level;
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};

use crate::{
    model::{Connections, Log, Proxies, Rules, Traffic, Version},
    ui::AsColor,
};
use crate::{Error, Result};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Event {
    Quit,
    Interface(Input),
    Update(UpdateEvent),
    Diagnostic(DiagnosticEvent),
}

impl<'a, 'b> From<&Event> for Spans<'a> {
    fn from(val: &Event) -> Self {
        match val {
            Event::Quit => Spans(vec![]),
            Event::Update(event) => Spans(vec![
                Span::styled(" ⇵  ", Style::default().fg(Color::Yellow)),
                Span::raw(event.to_string()),
            ]),
            Event::Interface(event) => Spans(vec![
                Span::styled(" ✜  ", Style::default().fg(Color::Green)),
                Span::raw(format!("{:?}", event)),
            ]),
            Event::Diagnostic(event) => match event {
                DiagnosticEvent::Log(level, payload) => Spans(vec![
                    Span::styled(
                        format!(" ✇  {:<6}", level),
                        Style::default().fg(level.as_color()),
                    ),
                    Span::raw(payload.to_owned()),
                ]),
            },
        }
    }
}

impl Event {
    pub fn is_quit(&self) -> bool {
        matches!(self, Event::Quit)
    }

    pub fn is_interface(&self) -> bool {
        matches!(self, Event::Interface(_))
    }

    pub fn is_update(&self) -> bool {
        matches!(self, Event::Update(_))
    }

    pub fn is_diagnostic(&self) -> bool {
        matches!(self, Event::Diagnostic(_))
    }
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
    Rules(Rules),
    Log(Log),
}

impl Display for UpdateEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateEvent::Connection(x) => write!(f, "{:?}", x),
            UpdateEvent::Version(x) => write!(f, "{:?}", x),
            UpdateEvent::Traffic(x) => write!(f, "{:?}", x),
            UpdateEvent::Proxies(x) => write!(f, "{:?}", x),
            UpdateEvent::Rules(x) => write!(f, "{:?}", x),
            UpdateEvent::Log(x) => write!(f, "{:?}", x),
        }
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum DiagnosticEvent {
    Log(Level, String),
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
