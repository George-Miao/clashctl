use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::sync::Mutex;
use std::time::Duration;
use std::{
    io::{self, Stdout},
    sync::Arc,
};

use clap::Parser;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use log::{LevelFilter, Record};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout, Rect};
use tui::{Frame, Terminal};

use crate::cli::ui::{
    pages::{ConfigPage, DebugPage, ProxiesPage, StatusPage},
    servo::servo,
};
use crate::cli::{components::*, ui::pages::LogPage, DiagnosticEvent, Event, Flags};
use crate::cli::{ui::utils::Interval, TuiStates};
use crate::{Error, Result};

type Backend = CrosstermBackend<Stdout>;

#[derive(Parser, Clone, Debug)]
pub struct TuiOpt {
    #[clap(
        short,
        long,
        default_value = "5",
        about = "Interval between delay updates, in seconds"
    )]
    interval: f32,
}

impl TuiOpt {
    pub fn into_app(self) -> Result<TuiApp> {
        Ok(TuiApp::from_opt(self))
    }

    pub fn run(self, flag: &Flags) -> Result<()> {
        self.into_app()?.run(flag)
    }
}

impl Default for TuiOpt {
    fn default() -> Self {
        Self { interval: 5.0 }
    }
}

#[derive(Debug, Default)]
pub struct TuiApp {
    opt: TuiOpt,
}

impl TuiApp {
    pub fn from_opt(opt: TuiOpt) -> Self {
        Self { opt }
    }

    fn setup() -> Result<Terminal<Backend>> {
        let mut stdout = io::stdout();

        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        Ok(terminal)
    }

    // TODO: wrap up
    fn wrap_up(mut terminal: Terminal<Backend>) -> Result<()> {
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;

        disable_raw_mode()?;

        Ok(())
    }

    pub fn run(&mut self, flag: &Flags) -> Result<()> {
        match flag.get_config()?.using_server() {
            Some(server) => server.to_owned(),
            None => {
                eprintln!("No server configured yet. Use `clashctl server add` first.");
                return Ok(());
            }
        };

        let opt = self.opt.clone();
        let flag = flag.clone();

        let mut terminal = Self::setup()?;

        match crossbeam::scope(move |s| {
            let (tx, rx) = channel();
            let state = Arc::new(Mutex::new(TuiStates::new()));
            let event_state = state.clone();

            s.spawn(move |_| servo(tx, &opt, &flag));
            s.spawn(move |_| loop {
                let mut state = event_state.lock().unwrap();

                match rx.try_recv() {
                    Ok(event) => match event {
                        Event::Quit => break,
                        _ => {
                            state.handle(event);
                        }
                    },
                    Err(TryRecvError::Disconnected) => {
                        eprintln!("All backend TX dropped");
                        break;
                    }
                    _ => {}
                }
            });
            s.spawn(move |_| {
                let mut interval = Interval::every(Duration::from_millis(10));

                loop {
                    interval.tick();
                    let mut state = match state.try_lock() {
                        Ok(state) => state,
                        Err(_) => continue,
                    };
                    state.new_tick();
                    terminal.draw(|f| Self::render(&mut state, f)).unwrap();
                }
            });
        }) {
            Err(e) => {
                eprintln!("{:?}", e);
                Err(Error::TuiInterupttedErr)
            }
            Ok(_) => Ok(()),
        }
    }

    fn route(state: &TuiStates, area: Rect, f: &mut Frame<Backend>) {
        match state.page_index {
            0 => (f.render_widget(StatusPage::new(state), area)),
            1 => (f.render_widget(ProxiesPage::new(state), area)),
            2 => (f.render_widget(LogPage::new(state), area)),
            3 => (f.render_widget(ConfigPage::new(state), area)),
            4 => (f.render_widget(DebugPage::new(state), area)),
            _ => unreachable!(),
        };
    }

    fn render(state: &TuiStates, f: &mut Frame<Backend>) {
        let layout = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(f.size());

        let tabs = Tabs::new(state);
        f.render_widget(tabs, layout[0]);

        let main = layout[1];

        Self::route(state, main, f);
    }
}

pub struct Logger {
    sender: Mutex<Sender<Event>>,
    level: LevelFilter,
}

impl Logger {
    pub fn new(sender: Sender<Event>) -> Self {
        Self::new_with_level(sender, LevelFilter::Info)
    }

    pub fn new_with_level(sender: Sender<Event>, level: LevelFilter) -> Self {
        Self {
            sender: Mutex::new(sender),
            level,
        }
    }

    pub fn apply(self) -> std::result::Result<(), log::SetLoggerError> {
        let level = self.level;
        log::set_boxed_logger(Box::new(self)).map(|()| log::set_max_level(level))
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        let text = format!(
            "{:<6}{:?}",
            record.level().to_string().to_uppercase(),
            record.args()
        );
        self.sender
            .lock()
            .unwrap()
            .send(Event::Diagnostic(DiagnosticEvent::Log(text)))
            .unwrap()
    }
    fn flush(&self) {}
}
