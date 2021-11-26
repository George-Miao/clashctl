use std::{
    cell::RefCell,
    io::{self, Stdout},
    sync::{mpsc::channel, Arc, Mutex, RwLock},
    thread::spawn,
    time::{Duration, Instant},
};

use clap::Parser;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use log::info;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout};
use tui::{Frame, Terminal};

use crate::{
    interactive::Flags,
    ui::{components::Tabs, servo, Interval, Logger, TicksCounter, TuiStates},
    Error, Result,
};

thread_local!(pub(crate) static TICK_COUNTER: RefCell<TicksCounter> = RefCell::new(TicksCounter::new_with_time(Instant::now())));

pub type Backend = CrosstermBackend<Stdout>;

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
    pub fn run(self, flag: Flags) -> Result<()> {
        main_loop(self, flag)
    }
}

impl Default for TuiOpt {
    fn default() -> Self {
        Self { interval: 5.0 }
    }
}

fn setup() -> Result<Terminal<Backend>> {
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    Ok(terminal)
}

fn wrap_up(mut terminal: Terminal<Backend>) -> Result<()> {
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;

    disable_raw_mode()?;

    Ok(())
}

pub fn main_loop(opt: TuiOpt, flag: Flags) -> Result<()> {
    if flag.get_config()?.using_server().is_none() {
        eprintln!("No server configured yet. Use `clashctl server add` first.");
        return Ok(());
    };

    let state = Arc::new(RwLock::new(TuiStates::new()));
    let error = Arc::new(Mutex::new(None));

    let (tx, rx) = channel();
    // let flag_clone = flag.clone();

    Logger::new(tx.clone()).apply()?;
    info!("Logger set");

    let back_handle = spawn(move || servo(tx, &opt, &flag));

    let event_state = state.clone();
    let event_error = error.clone();

    let handle = spawn(move || {
        while let Ok(event) = rx.recv() {
            let is_quit = event.is_quit();
            let mut state = event_state.write().unwrap();
            if let Err(e) = state.handle(event) {
                match event_error.lock() {
                    Ok(mut write) => write.replace(e),
                    Err(e) => panic!("Error: {}", e),
                };
                break;
            }
            if is_quit {
                break;
            }
        }
        event_state
            .write()
            .map(|mut x| x.should_quit = true)
            .unwrap();
    });

    let mut terminal = setup()?;

    let mut interval = Interval::every(Duration::from_millis(33));
    while let Ok(state) = state.read() {
        if !back_handle.is_running() {
            match back_handle.join() {
                Ok(_) => {}
                Err(e) => match error.lock() {
                    Ok(mut write) => {
                        write.replace(Error::Other(format!("{:?}", e)));
                    }
                    Err(e) => panic!("Error: {}", e),
                },
            }
            break;
        }
        if state.should_quit {
            break;
        }
        TICK_COUNTER.with(|t| t.borrow_mut().new_tick());
        if let Err(e) = terminal.draw(|f| render(&state, f)) {
            match error.lock() {
                Ok(mut write) => write.replace(e.into()),
                Err(e) => panic!("Error: {}", e),
            };
            break;
        }
        drop(state);
        interval.tick();
    }
    drop(handle);

    wrap_up(terminal)?;

    match error.lock() {
        Ok(mut guard) => {
            if let Some(error) = guard.take() {
                return Err(error);
            }
        }
        Err(e) => panic!("{}", e),
    }

    Ok(())
}

fn render(state: &TuiStates, f: &mut Frame<Backend>) {
    let layout = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.size());

    let tabs = Tabs::new(state);
    f.render_widget(tabs, layout[0]);

    let main = layout[1];

    state.route(main, f);
}
