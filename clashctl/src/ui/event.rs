use std::fmt::Display;

use clashctl_core::model::{ConnectionsWithSpeed, Log, Proxies, Rules, Traffic, Version};
use crossterm::event::{KeyCode as KC, KeyEvent as KE, KeyModifiers as KM};
use log::Level;
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};

use crate::{
    ui::{components::MovableListItem, utils::AsColor, TuiError, TuiResult},
    Action,
};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event {
    Quit,
    Action(Action),
    Input(InputEvent),
    Update(UpdateEvent),
    Diagnostic(DiagnosticEvent),
}

impl<'a> MovableListItem<'a> for Event {
    fn to_spans(&self) -> Spans<'a> {
        match self {
            Event::Quit => Spans(vec![]),
            Event::Action(action) => Spans(vec![
                Span::styled("⋉ ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{:?}", action)),
            ]),
            Event::Update(event) => Spans(vec![
                Span::styled("⇵  ", Style::default().fg(Color::Yellow)),
                Span::raw(event.to_string()),
            ]),
            Event::Input(event) => Spans(vec![
                Span::styled("✜  ", Style::default().fg(Color::Green)),
                Span::raw(format!("{:?}", event)),
            ]),
            Event::Diagnostic(event) => match event {
                DiagnosticEvent::Log(level, payload) => Spans(vec![
                    Span::styled(
                        format!("✇  {:<6}", level),
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
        matches!(self, Event::Input(_))
    }

    pub fn is_update(&self) -> bool {
        matches!(self, Event::Update(_))
    }

    pub fn is_diagnostic(&self) -> bool {
        matches!(self, Event::Diagnostic(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum InputEvent {
    Esc,
    TabGoto(u8),
    ToggleDebug,
    ToggleHold,
    List(ListEvent),
    TestLatency,
    NextSort,
    PrevSort,
    Other(KE),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListEvent {
    pub fast: bool,
    pub code: KC,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UpdateEvent {
    Config(crate::interactive::clashctl::model::Config),
    Connection(ConnectionsWithSpeed),
    Version(Version),
    Traffic(Traffic),
    Proxies(Proxies),
    Rules(Rules),
    Log(Log),
    ProxyTestLatencyDone,
}

impl Display for UpdateEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateEvent::Config(x) => write!(f, "{:?}", x),
            UpdateEvent::Connection(x) => write!(f, "{:?}", x),
            UpdateEvent::Version(x) => write!(f, "{:?}", x),
            UpdateEvent::Traffic(x) => write!(f, "{:?}", x),
            UpdateEvent::Proxies(x) => write!(f, "{:?}", x),
            UpdateEvent::Rules(x) => write!(f, "{:?}", x),
            UpdateEvent::Log(x) => write!(f, "{:?}", x),
            UpdateEvent::ProxyTestLatencyDone => write!(f, "Test latency done"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiagnosticEvent {
    Log(Level, String),
}

impl TryFrom<KC> for Event {
    type Error = TuiError;

    fn try_from(value: KC) -> TuiResult<Self> {
        match value {
            KC::Char('q') | KC::Char('x') => Ok(Event::Quit),
            KC::Char('t') => Ok(Event::Input(InputEvent::TestLatency)),
            KC::Esc => Ok(Event::Input(InputEvent::Esc)),
            KC::Char(' ') => Ok(Event::Input(InputEvent::ToggleHold)),
            KC::Char(char) if char.is_ascii_digit() => Ok(Event::Input(InputEvent::TabGoto(
                char.to_digit(10)
                    .expect("char.is_ascii_digit() should be able to parse into number")
                    as u8,
            ))),
            _ => Err(TuiError::TuiInternalErr),
        }
    }
}

impl From<KE> for Event {
    fn from(value: KE) -> Self {
        match (value.modifiers, value.code) {
            (KM::CONTROL, KC::Char('c')) => Self::Quit,
            (KM::CONTROL, KC::Char('d')) => Self::Input(InputEvent::ToggleDebug),
            (modi, arrow @ (KC::Left | KC::Right | KC::Up | KC::Down | KC::Enter)) => {
                Event::Input(InputEvent::List(ListEvent {
                    fast: matches!(modi, KM::CONTROL | KM::SHIFT),
                    code: arrow,
                }))
            }
            (KM::ALT, KC::Char('s')) => Self::Input(InputEvent::PrevSort),
            (KM::NONE, KC::Char('s')) => Self::Input(InputEvent::NextSort),
            (KM::NONE, key_code) => key_code
                .try_into()
                .unwrap_or(Self::Input(InputEvent::Other(value))),
            _ => Self::Input(InputEvent::Other(value)),
        }
    }
}
