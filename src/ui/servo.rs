use std::{sync::mpsc::Sender, thread::spawn, time::Duration};

use crossterm::event::Event as CrossTermEvent;

use log::warn;

use crate::{
    interactive::Flags,
    ui::{
        utils::{Check, Interval, Pulse},
        Event, TuiOpt, UpdateEvent,
    },
    Error, Result,
};

pub(super) fn servo(tx: Sender<Event>, opt: &TuiOpt, flags: &Flags) -> Result<()> {
    macro_rules! run {
        ($(let $p:pat = $v:expr;)* $block:block) => {
            {
                $(let $p = $v;)*
                Some(spawn(move || -> Result<()> { $block }))
            }
        }
    }

    let clash = flags.connect_server_from_config()?;

    let mut key_handle = run!(
        let key_tx = tx.clone();
        {
            loop {
                match crossterm::event::read() {
                    Ok(CrossTermEvent::Key(event)) => key_tx.send(Event::from(event))?,
                    Err(_) => {
                        key_tx.send(Event::Quit)?;
                        break;
                    }
                    _ => {}
                }
            }
            Ok(())
        }
    );

    let mut req_handle = run!(
        let tx = tx.clone();
        let req_clash = clash.clone();
        {
            let mut interval = Interval::every(Duration::from_millis(50));
            let mut connection_pulse = Pulse::new(20); // Every 1 s
            let mut proxies_pulse = Pulse::new(100); // Every 5 s
            let mut rules_pulse = Pulse::new(100); // Every 5 s
            let mut version_pulse = Pulse::new(100); // Every 5 s

            let clash = req_clash;
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
        }
    );

    let mut traffic_handle = run!(
        let traffic_tx = tx.clone();
        let mut traffics = clash.get_traffic()?;
    {
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

    let mut log_handle = run!(
        let log_tx = tx.clone();
        let mut logs = clash.get_log()?;
        {
            loop {
                match logs.next() {
                    Some(Ok(log)) => log_tx.send(Event::Update(UpdateEvent::Log(log)))?,
                    Some(Err(e)) => warn!("{:?}", e),
                    None => warn!("No more traffic"),
                }
            }
        }
    );

    let mut interval = Interval::every(Duration::from_millis(100));
    loop {
        interval.tick();
        if !(key_handle.check("key")
            || traffic_handle.check("traffic")
            || log_handle.check("log")
            || req_handle.check("request"))
        {
            break;
        }
    }

    // All backend tasks dead - indicates error
    Err(Error::TuiBackendErr)
}
