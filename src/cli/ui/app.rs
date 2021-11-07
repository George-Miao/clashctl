use std::io::{self, Stdout};
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

use clap::Parser;
use crossterm::event::Event as CrossTermEvent;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::{Frame, Terminal};

use crate::cli::ui::components::TabState;
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

#[derive(Clone, Debug)]
pub struct TuiStates {
    tab_state: TabState,
    ticks: u64,
    start_time: Instant,
    number_of_events: u64,
}

impl Default for TuiStates {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            number_of_events: Default::default(),
            tab_state: Default::default(),
            ticks: Default::default(),
        }
    }
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

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let (tx, rx) = channel();
        let opt = self.opt.clone();
        let handle = spawn(move || Self::servo(tx, &opt));

        let stdout = io::stdout();
        // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        loop {
            self.state.ticks += 1;
            terminal.draw(|f| self.render(f))?;
            if !handle.is_running() {
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

        // execute!(
        //     terminal.backend_mut(),
        //     LeaveAlternateScreen,
        //     DisableMouseCapture
        // )?;
        disable_raw_mode()?;
        Ok(())
    }

    fn render(&mut self, f: &mut Frame<Backend>) {
        let main_layout = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(f.size());

        let tabs = Tabs::default();
        f.render_stateful_widget(tabs, main_layout[0], &mut self.state.tab_state);

        let info_layout = Layout::default()
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .direction(Direction::Horizontal)
            .split(main_layout[1]);

        let block = Block::default().title("Debug").borders(Borders::ALL);
        let tps = self.state.ticks as f64 / self.state.start_time.elapsed().as_secs_f64();
        let text = Paragraph::new(format!(
            "Ticks: {}\nTPS: {:.2}\nEvents: {}",
            self.state.ticks, tps, self.state.number_of_events
        ))
        .block(block)
        .wrap(Wrap { trim: true });

        f.render_widget(text, info_layout[0]);
    }

    fn handle(&mut self, _t: &mut Terminal<Backend>, event: &Event) -> Result<()> {
        match event {
            Event::Quit => Err(Error::TuiInterupttedErr),
            Event::Traffic(_traffic) => Ok(()),
            Event::Log(_log) => Ok(()),
            Event::TabNext | Event::TabPrev => self.state.tab_state.handle(event),
            _ => Ok(()),
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
