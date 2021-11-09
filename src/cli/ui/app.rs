use std::io::{self, Stdout};
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::sync::Mutex;
use std::thread::spawn;
use std::time::Duration;

use clap::Parser;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event as CrossTermEvent};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use log::{info, warn, LevelFilter, Record};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout, Rect};
use tui::{Frame, Terminal};

use crate::cli::{components::*, Event, Flags};
use crate::cli::{
    ui::pages::{ConfigPage, DebugPage, ProxiesPage, StatusPage},
    UpdateEvent,
};
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

    pub fn run(&mut self, flag: &Flags) -> Result<()> {
        match flag.get_config()?.using_server() {
            Some(server) => server.to_owned(),
            None => {
                eprintln!("No server configured yet. Use `clashctl server add` first.");
                return Ok(());
            }
        };

        let (tx, rx) = channel();
        let opt = self.opt.clone();
        let flag = flag.clone();
        let servo_handle = spawn(move || Self::servo(tx, &opt, &flag));

        let mut terminal = Self::setup()?;

        let mut interval = Interval::every(Duration::from_millis(10));

        loop {
            interval.tick();
            // self.state.debug_state.new_tick();
            terminal.draw(|f| self.render(f))?;
            if !servo_handle.is_running() {
                Self::wrap_up(terminal)?;
                eprintln!("Servo dropped: {:?}", servo_handle.join().unwrap());
                break;
            }
            match rx.try_recv() {
                Ok(event) => {
                    if let Err(e) = self.handle(&event) {
                        Self::wrap_up(terminal)?;
                        eprintln!("Quit: {}", e);
                        break;
                    }
                }
                Err(TryRecvError::Disconnected) => {
                    Self::wrap_up(terminal)?;
                    eprintln!("All backend TX dropped");
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn route(&mut self, area: Rect, f: &mut Frame<Backend>) {
        match self.state.page_index {
            0 => {
                let page = StatusPage::default();
                f.render_stateful_widget(page, area, &mut self.state)
            }
            1 => {
                let page = ProxiesPage::default();
                f.render_stateful_widget(page, area, &mut self.state)
            }
            2 => {
                let page = ConfigPage::default();
                f.render_stateful_widget(page, area, &mut self.state)
            }
            3 => {
                let page = DebugPage::default();
                f.render_stateful_widget(page, area, &mut self.state)
            }
            _ => unreachable!(),
        }
    }

    fn render(&mut self, f: &mut Frame<Backend>) {
        let layout = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(f.size());

        let tabs = Tabs::default();
        f.render_stateful_widget(tabs, layout[0], &mut self.state);

        let main = layout[1];

        self.route(main, f);
    }

    fn handle(&mut self, event: &Event) -> Result<()> {
        match event {
            Event::Quit => Err(Error::TuiInterupttedErr),
            Event::Log(_log) => Ok(()),
            _ => self.state.handle(event),
        }
    }

    fn servo(tx: Sender<Event>, opt: &TuiOpt, flags: &Flags) -> Result<()> {
        macro_rules! run {
            ($block:block) => {
                Some(spawn(move || -> Result<()> {
                    $block
                    Ok(())
                }))
            }
        }

        macro_rules! watch {
            ($identifier:literal, $handle:ident) => {
                if let Some(ref handle) = $handle {
                    if !handle.is_running() {
                        let handle = $handle.take().unwrap();
                        match handle.join() {
                            Ok(res) => warn!(
                                "Background task `{}` has stopped running ({:?})",
                                $identifier, res
                            ),
                            Err(e) => warn!(
                                "Background task `{}` has stopped running\n ({:?})",
                                $identifier, e
                            ),
                        }
                    }
                }
            };
        }

        Logger::new(tx.clone()).apply().unwrap();

        info!("Logger set");

        let key_tx = tx.clone();
        let traffic_tx = tx.clone();
        let _req_tx = tx;

        let clash = flags.connect_server_from_config()?;
        // let traffics = clash.get_traffic()?;

        let interval_ms = (opt.interval * 1000f32) as u64;
        let _interval = Duration::from_millis(interval_ms);

        let mut key_handle = run!({
            loop {
                match crossterm::event::read() {
                    Ok(CrossTermEvent::Key(event)) => {
                        if let Ok(event) = Event::try_from(event) {
                            key_tx.send(event)?
                        }
                    }

                    Err(_) => {
                        key_tx.send(Event::Quit)?;
                        break;
                    }
                    _ => {}
                }
            }
        });

        #[allow(unreachable_code)]
        let mut traffic_handle = run!({
            let mut traffics = clash.get_traffic()?;
            loop {
                match traffics.next() {
                    Some(Ok(traffic)) => {
                        traffic_tx.send(Event::Update(UpdateEvent::Traffic(traffic)))?
                    }
                    // Some(Ok(traffic)) => info!("{}", traffic),
                    Some(Err(e)) => warn!("{:?}", e),
                    None => warn!("No more traffic"),
                }
            }
        });

        #[allow(unreachable_code)]
        #[allow(clippy::empty_loop)]
        let mut req_handle = run!({
            loop {
                // sleep(interval);
                // req_tx.send(Event::Update)?;
            }
        });

        loop {
            watch!("key", key_handle);
            watch!("traffic", traffic_handle);
            watch!("request", req_handle);
        }
    }
}

pub struct Logger {
    sender: Mutex<Sender<Event>>,
    level: LevelFilter,
}

impl Logger {
    pub fn new(sender: Sender<Event>) -> Self {
        Self::new_with_level(sender, LevelFilter::Debug)
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
        let text = format!("{:?}", record.args());
        self.sender.lock().unwrap().send(Event::Log(text)).unwrap()
    }
    fn flush(&self) {}
}
