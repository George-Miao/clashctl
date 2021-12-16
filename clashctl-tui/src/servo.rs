use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
    thread::{spawn, JoinHandle},
    time::Duration,
};

use clashctl_interactive::Flags;
use crossterm::event::Event as CrossTermEvent;
use log::warn;
use rayon::prelude::*;

use crate::{
    clashctl::Clash,
    event::{Event, UpdateEvent},
    utils::{Interval, Pulse},
    Action, Result, TuiOpt,
};

pub(crate) trait Check {
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

pub type Job = JoinHandle<Result<()>>;

#[derive(Debug)]
pub struct Servo {
    traffic_handle: Option<Job>,
    input_handle: Option<Job>,
    req_handle: Option<Job>,
    log_handle: Option<Job>,
    action_handle: Option<Job>,
}

// TODO change behavior based on opt
// rely on config
impl Servo {
    pub fn run(
        tx: Sender<Event>,
        rx: Receiver<Action>,
        opt: Arc<TuiOpt>,
        flags: Arc<Flags>,
    ) -> Result<Self> {
        let clash = flags.connect_server_from_config()?;
        clash.get_version()?;
        let clash = Arc::new(clash);
        let this = Self {
            input_handle: Some(Self::input_job(tx.clone())),
            traffic_handle: Some(Self::traffic_job(tx.clone(), clash.clone())),
            log_handle: Some(Self::log_job(tx.clone(), clash.clone())),
            req_handle: Some(Self::req_job(
                opt.clone(),
                flags.clone(),
                tx.clone(),
                clash.clone(),
            )),
            action_handle: Some(Self::action_job(opt, flags, tx, rx, clash)),
        };
        Ok(this)
    }

    fn input_job(tx: Sender<Event>) -> Job {
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

    fn req_job(_opt: Arc<TuiOpt>, _flags: Arc<Flags>, tx: Sender<Event>, clash: Arc<Clash>) -> Job {
        spawn(move || {
            let mut interval = Interval::every(Duration::from_millis(50));
            let mut connection_pulse = Pulse::new(20); // Every 1 s
            let mut proxies_pulse = Pulse::new(100); //   Every 5 s + 0 tick
            let mut rules_pulse = Pulse::new(101); //     Every 5 s + 1 tick
            let mut version_pulse = Pulse::new(102); //   Every 5 s + 2 tick
            let mut config_pulse = Pulse::new(103); //    Every 5 s + 3 tick

            loop {
                if version_pulse.tick() {
                    tx.send(Event::Update(UpdateEvent::Version(clash.get_version()?)))?;
                }
                if connection_pulse.tick() {
                    tx.send(Event::Update(UpdateEvent::Connection(
                        clash.get_connections()?.into(),
                    )))?;
                }
                if rules_pulse.tick() {
                    tx.send(Event::Update(UpdateEvent::Rules(clash.get_rules()?)))?;
                }
                if proxies_pulse.tick() {
                    tx.send(Event::Update(UpdateEvent::Proxies(clash.get_proxies()?)))?;
                }
                if config_pulse.tick() {
                    tx.send(Event::Update(UpdateEvent::Config(clash.get_configs()?)))?;
                }
                interval.tick();
            }
        })
    }

    fn traffic_job(tx: Sender<Event>, clash: Arc<Clash>) -> Job {
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

    fn log_job(tx: Sender<Event>, clash: Arc<Clash>) -> Job {
        spawn(move || loop {
            let mut logs = clash.get_log()?;
            match logs.next() {
                Some(Ok(log)) => tx.send(Event::Update(UpdateEvent::Log(log)))?,
                Some(Err(e)) => warn!("{:?}", e),
                None => warn!("No more traffic"),
            }
        })
    }

    fn action_job(
        opt: Arc<TuiOpt>,
        flags: Arc<Flags>,
        tx: Sender<Event>,
        rx: Receiver<Action>,
        clash: Arc<Clash>,
    ) -> Job {
        spawn(move || {
            while let Ok(action) = rx.recv() {
                match action {
                    Action::TestLatency { proxies } => {
                        let result = proxies
                            .par_iter()
                            .filter_map(|proxy| {
                                clash
                                    .get_proxy_delay(proxy, flags.test_url.as_str(), flags.timeout)
                                    .err()
                            })
                            .collect::<Vec<_>>();

                        let count = result.len();

                        if count != 0 {
                            warn!(
                                "   {}",
                                result
                                    .into_iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            );
                            warn!("({}) error(s) during test proxy delay", count);
                        }

                        tx.send(Event::Update(UpdateEvent::ProxyTestLatencyDone))?;
                        tx.send(Event::Update(UpdateEvent::Proxies(clash.get_proxies()?)))?;
                    }
                    Action::ApplySelection { group, proxy } => {
                        let _ = clash
                            .set_proxygroup_selected(&group, &proxy)
                            .map_err(|e| warn!("{:?}", e));
                        tx.send(Event::Update(UpdateEvent::Proxies(clash.get_proxies()?)))?;
                    }
                }
            }
            Ok(())
        })
    }
}

impl Check for Servo {
    fn ok(&mut self, _: &str) -> bool {
        self.input_handle.ok("key")
            && self.traffic_handle.ok("traffic")
            && self.log_handle.ok("log")
            && self.req_handle.ok("request")
            && self.action_handle.ok("action")
    }
}
