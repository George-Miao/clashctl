use std::io::{self, Stdout};
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread::{sleep, spawn};
use std::time::Duration;

use clap::Parser;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event as CrossTermEvent};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout, Rect};
use tui::{Frame, Terminal};

use crate::cli::ui::components::TabState;
use crate::cli::ui::pages::{
    ConfigPage, ConfigState, ProxiesPage, ProxiesState, StatusPage, StatusState,
};
use crate::cli::{components::*, Event, EventHandler};
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

    pub fn run(self) -> Result<()> {
        self.into_app()?.run()
    }
}

impl Default for TuiOpt {
    fn default() -> Self {
        Self { interval: 5.0 }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TuiStates {
    tab_state: TabState,
    proxies_state: ProxiesState,
    status_state: StatusState,
    config_state: ConfigState,
    ticks: u64,
    number_of_events: u64,
}

#[derive(Clone, Debug, Default)]
pub struct TuiApp {
    opt: TuiOpt,
    state: TuiStates,
}

impl TuiApp {
    pub fn from_opt(opt: TuiOpt) -> Self {
        Self {
            opt,
            ..Default::default()
        }
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

    fn wrap_up(mut terminal: Terminal<Backend>) -> Result<()> {
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;

        disable_raw_mode()?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        let (tx, rx) = channel();
        let opt = self.opt.clone();
        let servo_handle = spawn(move || Self::servo(tx, &opt));

        let mut terminal = Self::setup()?;

        loop {
            self.state.ticks += 1;
            terminal.draw(|f| self.render(f))?;
            if !servo_handle.is_running() {
                return Err(Error::TuiBackendErr);
            }
            match rx.try_recv() {
                Ok(event) => {
                    self.state.number_of_events += 1;
                    if self.handle(&mut terminal, &event).is_err() {
                        break;
                    }
                }
                Err(TryRecvError::Disconnected) => break,
                _ => {}
            }
        }

        Self::wrap_up(terminal)?;

        Ok(())
    }

    fn route(&mut self, area: Rect, f: &mut Frame<Backend>) {
        match self.state.tab_state.index {
            0 => {
                let page = StatusPage::default();
                f.render_stateful_widget(page, area, &mut self.state.status_state)
            }
            1 => {
                let page = ProxiesPage::default();
                f.render_stateful_widget(page, area, &mut self.state.proxies_state)
            }
            2 => {
                let page = ConfigPage::default();
                f.render_stateful_widget(page, area, &mut self.state.config_state)
            }
            _ => unreachable!(),
        }
    }

    fn render(&mut self, f: &mut Frame<Backend>) {
        let layout = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(f.size());

        let tabs = Tabs::default();
        f.render_stateful_widget(tabs, layout[0], &mut self.state.tab_state);

        let main = layout[1];

        self.route(main, f);
    }

    fn handle(&mut self, _t: &mut Terminal<Backend>, event: &Event) -> Result<()> {
        match event {
            Event::Quit => Err(Error::TuiInterupttedErr),
            Event::Traffic(_traffic) => Ok(()),
            Event::Log(_log) => Ok(()),
            Event::TabNext | Event::TabPrev => self.state.tab_state.handle(event),
            Event::Update => self.state.proxies_state.handle(event),
            // _ => Ok(()),
        }
    }

    fn servo(tx: Sender<Event>, opt: &TuiOpt) -> Result<()> {
        let key_tx = tx.clone();
        let key_handle = spawn(move || -> Result<()> {
            loop {
                match crossterm::event::read() {
                    Ok(CrossTermEvent::Key(event)) => {
                        if let Ok(event) = Event::try_from(event) {
                            key_tx.send(event).map_err(|_| Error::TuiBackendErr)?
                        }
                    }

                    Err(_) => {
                        key_tx.send(Event::Quit).map_err(|_| Error::TuiBackendErr)?;
                        break;
                    }
                    _ => {}
                }
            }
            Ok(())
        });

        let interval_ms = (opt.interval * 1000f32) as u64;
        let interval = Duration::from_millis(interval_ms);

        let request_handle = spawn(move || -> Result<()> {
            // let clash = Clash::new();

            loop {
                sleep(interval);
                tx.send(Event::Update).map_err(|_| Error::TuiBackendErr)?;
            }
        });

        let is_running = || key_handle.is_running() && request_handle.is_running();
        while is_running() {
            sleep(Duration::from_millis(10))
        }
        Ok(())
    }
}
