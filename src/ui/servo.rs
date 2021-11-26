use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
    thread::{spawn, JoinHandle},
    time::Duration,
};

use crossterm::event::Event as CrossTermEvent;

use log::warn;

use crate::{
    interactive::Flags,
    ui::{
        app::TuiOpt,
        event::{Event, UpdateEvent},
        utils::{Interval, Pulse},
        Action,
    },
    Clash, Result,
};

pub trait Check {
    fn ok(&mut self, indent: &str) -> bool;
}

impl<T: std::fmt::Debug> Check for Option<JoinHandle<T>> {
    fn ok(&mut self, indent: &str) -> bool {
        if let Some(ref handle) = self {
            if !handle.is_running() {
                let handle = self.take().unwrap();
                match handle.join() {
                    Ok(res) => warn!(
                        "Background task `{}` has stopped running ({:?})",
                        indent, res
                    ),
                    Err(e) => warn!(
                        "Catastrophic failure: Background task `{}` has stopped running ({:?})",
                        indent, e
                    ),
                }
                // Not running anymore
                false
            } else {
                // Running properly
                true
            }
        } else {
            // Already quit and handled earlier
            false
        }
    }
}

#[derive(Debug)]
pub struct Servo {
    traffic_handle: Option<JoinHandle<Result<()>>>,
    input_handle: Option<JoinHandle<Result<()>>>,
    req_handle: Option<JoinHandle<Result<()>>>,
    log_handle: Option<JoinHandle<Result<()>>>,
}

// TODO change behavior based on opt
// rely on config
impl Servo {
    pub fn run(
        tx: Sender<Event>,
        rx: Receiver<Action>,
        opt: &TuiOpt,
        flags: &Flags,
    ) -> Result<Self> {
        let clash = flags.connect_server_from_config()?;
        let clash = Arc::new(clash);
        let this = Self {
            traffic_handle: Some(Self::traffic_job(tx.clone(), clash.clone())),
            req_handle: Some(Self::req_job(tx.clone(), clash.clone())),
            log_handle: Some(Self::log_job(tx.clone(), clash)),
            input_handle: Some(Self::input_job(tx)),
        };
        Ok(this)
    }

    fn input_job(tx: Sender<Event>) -> JoinHandle<Result<()>> {
        spawn(move || {
            loop {
                match crossterm::event::read() {
                    Ok(CrossTermEvent::Key(event)) => tx.send(Event::from(event))?,
                    Err(_) => {
                        tx.send(Event::Quit)?;
                        break;
                    }
                    _ => {}
                }
            }
            Ok(())
        })
    }

    fn req_job(tx: Sender<Event>, clash: Arc<Clash>) -> JoinHandle<Result<()>> {
        spawn(move || {
            let mut interval = Interval::every(Duration::from_millis(50));
            let mut connection_pulse = Pulse::new(20); // Every 1 s
            let mut proxies_pulse = Pulse::new(100); // Every 5 s
            let mut rules_pulse = Pulse::new(100); // Every 5 s
            let mut version_pulse = Pulse::new(100); // Every 5 s

            loop {
                if version_pulse.tick() {
                    tx.send(Event::Update(UpdateEvent::Version(clash.get_version()?)))?;
                }
                if connection_pulse.tick() {
                    tx.send(Event::Update(UpdateEvent::Connection(
                        clash.get_connections()?,
                    )))?;
                }
                if rules_pulse.tick() {
                    tx.send(Event::Update(UpdateEvent::Rules(clash.get_rules()?)))?;
                }
                if proxies_pulse.tick() {
                    tx.send(Event::Update(UpdateEvent::Proxies(clash.get_proxies()?)))?;
                }
                interval.tick();
            }
        })
    }

    fn traffic_job(tx: Sender<Event>, clash: Arc<Clash>) -> JoinHandle<Result<()>> {
        spawn(move || {
            let mut traffics = clash.get_traffic()?;
            loop {
                match traffics.next() {
                    Some(Ok(traffic)) => tx.send(Event::Update(UpdateEvent::Traffic(traffic)))?,
                    // Some(Ok(traffic)) => info!("{}", traffic),
                    Some(Err(e)) => warn!("{:?}", e),
                    None => warn!("No more traffic"),
                }
            }
        })
    }

    fn log_job(tx: Sender<Event>, clash: Arc<Clash>) -> JoinHandle<Result<()>> {
        spawn(move || loop {
            let mut logs = clash.get_log()?;
            match logs.next() {
                Some(Ok(log)) => tx.send(Event::Update(UpdateEvent::Log(log)))?,
                Some(Err(e)) => warn!("{:?}", e),
                None => warn!("No more traffic"),
            }
        })
    }
}

impl Check for Servo {
    fn ok(&mut self, _: &str) -> bool {
        self.input_handle.ok("key")
            && self.traffic_handle.ok("traffic")
            && self.log_handle.ok("log")
            && self.req_handle.ok("request")
    }
}
