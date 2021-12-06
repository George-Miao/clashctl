use std::{
    cell::RefCell,
    io::{self, Stdout},
    sync::{mpsc::channel, Arc, Mutex, RwLock},
    thread::spawn,
    time::{Duration, Instant},
};

use crate::{
    components::Tabs, pages::route, Check, Interval, Logger, Result, Servo, TicksCounter, TuiStates,
};
// use clap::Parser;
use clashctl_interactive::{Flags, TuiOpt};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use log::info;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout};
use tui::{Frame, Terminal};

thread_local!(pub(crate) static TICK_COUNTER: RefCell<TicksCounter> = RefCell::new(TicksCounter::new_with_time(Instant::now())));

pub type Backend = CrosstermBackend<Stdout>;

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

    let state = Arc::new(RwLock::new(TuiStates::default()));
    let error = Arc::new(Mutex::new(None));

    let (event_tx, event_rx) = channel();
    let (action_tx, action_rx) = channel();

    Logger::new(event_tx.clone()).apply()?;
    info!("Logger set");
    let opt = Arc::new(opt);
    let flag = Arc::new(flag);

    let mut servo = Servo::run(event_tx, action_rx, opt, flag)?;

    let event_handler_state = state.clone();
    let event_handler_error = error.clone();

    let handle = spawn(move || {
        let mut should_quit;
        while let Ok(event) = event_rx.recv() {
            should_quit = event.is_quit();
            let mut state = event_handler_state.write().unwrap();
            match state.handle(event) {
                Ok(Some(action)) => {
                    if let Err(e) = action_tx.send(action) {
                        event_handler_error.lock().unwrap().replace(e.into());
                        should_quit = true;
                    }
                }
                // No action needed
                Ok(None) => {}
                Err(e) => {
                    event_handler_error.lock().unwrap().replace(e);
                    should_quit = true;
                }
            }
            if should_quit {
                break;
            }
        }
        event_handler_state
            .write()
            .map(|mut x| x.should_quit = true)
            .unwrap();
    });

    let mut terminal = setup()?;

    let mut interval = Interval::every(Duration::from_millis(33));
    while let Ok(state) = state.read() {
        if !handle.is_running() {
            break;
        }

        if !servo.ok("servo") {
            break;
        }

        if state.should_quit {
            break;
        }

        TICK_COUNTER.with(|t| t.borrow_mut().new_tick());
        if let Err(e) = terminal.draw(|f| render(&state, f)) {
            error.lock().unwrap().replace(e.into());
            break;
        }
        drop(state);
        interval.tick();
    }
    drop(servo);
    drop(handle);

    wrap_up(terminal)?;

    if let Some(error) = error.lock().unwrap().take() {
        return Err(error);
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

    route(state, main, f);
}
