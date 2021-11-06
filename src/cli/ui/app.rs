use std::io::{self, Stdout};
use std::sync::mpsc::{channel, Sender};
use std::thread::{sleep, spawn};
use std::time::Duration;

use clap::Parser;
use crossterm::terminal::disable_raw_mode;
use crossterm::{
    event::{Event as CrossTermEvent, KeyCode},
    terminal::enable_raw_mode,
};
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders};
use tui::{Frame, Terminal};

use crate::cli::Event;
use crate::{Error, Result};

type Backend = CrosstermBackend<Stdout>;

#[derive(Parser, Clone, Debug, Default)]
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

pub struct TuiApp {
    opt: TuiOpt,
}

impl TuiApp {
    pub fn from_opt(opt: TuiOpt) -> Self {
        Self { opt }
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let (tx, rx) = channel();
        let opt = self.opt.clone();
        let _handle = spawn(move || Self::servo(tx, &opt));

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        loop {
            terminal.draw(|f| self.render(f))?;
            let event = rx.recv().map_err(|_| Error::TuiBackendErr)?;
            if !self.handle(&mut terminal, event) {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }

    fn render(&self, f: &mut Frame<Backend>) {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);

        f.render_widget(block, size);
    }

    fn handle(&mut self, _t: &mut Terminal<Backend>, event: Event) -> bool {
        match event {
            Event::Quit => false,
            Event::Traffic(_traffic) => true,
            Event::Log(_log) => true,
            _ => true,
        }
    }

    fn servo(tx: Sender<Event>, opt: &TuiOpt) -> Result<()> {
        let key_tx = tx.clone();
        let ct_event_handle = spawn(move || -> Result<()> {
            loop {
                // `read()` blocks until an `Event` is available
                match crossterm::event::read() {
                    Ok(CrossTermEvent::Key(event)) => match event.code {
                        KeyCode::Char('q') => {
                            key_tx.send(Event::Quit).map_err(|_| Error::TuiBackendErr)?;
                            break;
                        }
                        KeyCode::Char('x') => {
                            key_tx.send(Event::Quit).map_err(|_| Error::TuiBackendErr)?;
                            break;
                        }
                        _ => {}
                    },
                    Ok(CrossTermEvent::Resize(width, height)) => {
                        println!("New size {}x{}", width, height)
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

        let is_running = || ct_event_handle.is_running() && request_handle.is_running();
        while is_running() {
            sleep(Duration::from_millis(10))
        }
        Ok(())
    }
}
