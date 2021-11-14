use std::{
    cell::RefCell,
    io::{self, Stdout},
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};
use std::{sync::mpsc::channel, thread::spawn};

use clap::Parser;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout, Rect};
use tui::{Frame, Terminal};

use crate::cli::Flags;
use crate::ui::{
    components::*,
    pages::{ConfigPage, ConnectionsPage, DebugPage, LogPage, ProxiesPage, RulesPage, StatusPage},
    servo::servo,
    utils::{Interval, Logger, TicksCounter},
    TuiStates,
};
use crate::Result;

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
    pub fn run(self, flag: &Flags) -> Result<()> {
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

pub fn main_loop(opt: TuiOpt, flag: &Flags) -> Result<()> {
    if flag.get_config()?.using_server().is_none() {
        eprintln!("No server configured yet. Use `clashctl server add` first.");
        return Ok(());
    };

    let mut terminal = setup()?;

    let (tx, rx) = channel();
    let flag_clone = flag.clone();

    Logger::new(tx.clone()).apply()?;

    spawn(move || servo(tx, &opt, &flag_clone));

    let state = Arc::new(RwLock::new(TuiStates::new()));

    let event_state = state.clone();

    spawn(move || {
        while let Ok(event) = rx.recv() {
            match event_state.write() {
                Ok(mut writer) => {
                    if writer.handle(event).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            };
        }
    });

    let mut interval = Interval::every(Duration::from_millis(30));
    while let Ok(state) = state.read() {
        interval.tick();
        TICK_COUNTER.with(|t| t.borrow_mut().new_tick());
        if terminal.draw(|f| render(&state, f)).is_err() {
            break;
        }
    }
    // state.new_tick();
    // match rx.try_recv() {
    //     Ok(event) => match event {
    //         Event::Quit => break,
    //         _ => state.handle(event)?,
    //     },
    //     Err(TryRecvError::Disconnected) => {
    //         eprintln!("All backend TX dropped");
    //         break;
    //     }
    //     _ => {}
    // }

    wrap_up(terminal)?;

    Ok(())
}

fn render(state: &TuiStates, f: &mut Frame<Backend>) {
    let layout = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.size());

    let tabs = Tabs::new(state);
    f.render_widget(tabs, layout[0]);

    let main = layout[1];

    state.render_route(main, f);
}
