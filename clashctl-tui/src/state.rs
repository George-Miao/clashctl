use std::{collections::HashMap, time::Instant};

use clashctl_interactive::{Noop, RuleSort};
use smart_default::SmartDefault;

use crate::{
    clashctl::model::{ConnectionWithSpeed, Log, Rule, Traffic, Version},
    components::{MovableListManage, MovableListManager, MovableListState, ProxyTree},
    Action, ConfigState, Event, InputEvent, Result, UpdateEvent,
};

pub(crate) type LogListState<'a> = MovableListState<'a, Log, Noop>;
pub(crate) type ConListState<'a> = MovableListState<'a, ConnectionWithSpeed, Noop>;
pub(crate) type RuleListState<'a> = MovableListState<'a, Rule, RuleSort>;
pub(crate) type DebugListState<'a> = MovableListState<'a, Event, Noop>;

#[derive(Debug, Clone, SmartDefault)]
pub struct TuiStates<'a> {
    pub should_quit: bool,
    #[default(_code = "Instant::now()")]
    pub start_time: Instant,
    pub version: Option<Version>,
    pub traffics: Vec<Traffic>,
    pub max_traffic: Traffic,
    pub all_events_recv: usize,
    pub page_index: u8,
    pub show_debug: bool,
    pub proxy_tree: ProxyTree<'a>,
    pub rule_freq: HashMap<String, usize>,
    // (upload_size, download_size)
    pub con_size: (u64, u64),

    #[default(_code = "{
        let mut ret = MovableListState::default();
        ret.with_index().dsc_index();
        ret
    }")]
    pub log_state: LogListState<'a>,
    pub con_state: ConListState<'a>,
    pub rule_state: RuleListState<'a>,
    pub debug_state: DebugListState<'a>,
    pub config_state: ConfigState,
}

// TODO fix: drop_events not working
impl<'a> TuiStates<'a> {
    pub const TITLES: &'static [&'static str] = &[
        "Status", "Proxies", "Rules", "Conns", "Logs", "Configs", "Debug",
    ];

    pub fn handle(&mut self, event: Event) -> Result<Option<Action>> {
        self.all_events_recv += 1;
        if self.debug_state.len() >= 300 {
            let _ = self.drop_events(100);
        }
        self.debug_state.push(event.to_owned());

        match event {
            Event::Quit => {
                self.should_quit = true;
                Ok(None)
            }
            Event::Input(event) => self.handle_input(event),
            Event::Update(update) => self.handle_update(update),
            _ => Ok(None),
        }
    }

    #[inline]
    pub fn page_len(&mut self) -> usize {
        if self.show_debug {
            Self::TITLES.len()
        } else {
            Self::TITLES.len() - 1
        }
    }

    #[inline]
    pub fn title(&self) -> &str {
        Self::TITLES[self.page_index as usize]
    }

    fn active_list<'own>(&'own mut self) -> Option<MovableListManager<'a, 'own>> {
        match self.title() {
            "Rules" => Some(MovableListManager::Rule(&mut self.rule_state)),
            "Debug" => Some(MovableListManager::Event(&mut self.debug_state)),
            "Logs" => Some(MovableListManager::Log(&mut self.log_state)),
            "Conns" => Some(MovableListManager::Connection(&mut self.con_state)),
            "Proxies" => Some(MovableListManager::Proxy(&mut self.proxy_tree)),
            _ => None,
        }
    }

    fn handle_update(&mut self, update: UpdateEvent) -> Result<Option<Action>> {
        match update {
            UpdateEvent::Config(config) => self.config_state.update_clash(config),
            UpdateEvent::Connection(connection) => {
                self.con_size = (connection.upload_total, connection.download_total);
                self.con_state.sorted_merge(connection.connections);
                self.con_state.with_index();
            }
            UpdateEvent::Version(version) => self.version = Some(version),
            UpdateEvent::Traffic(traffic) => {
                let Traffic { up, down } = traffic;
                self.max_traffic.up = self.max_traffic.up.max(up);
                self.max_traffic.down = self.max_traffic.down.max(down);
                self.traffics.push(traffic)
            }
            UpdateEvent::Proxies(proxies) => {
                let mut new_tree = Into::<ProxyTree>::into(proxies);
                new_tree.sort_groups_with_frequency(&self.rule_freq);
                self.proxy_tree.replace_with(new_tree);
            }
            UpdateEvent::Log(log) => self.log_state.push(log),
            UpdateEvent::Rules(rules) => {
                self.rule_freq = rules.owned_frequency();
                self.rule_state.sorted_merge(rules.rules);
            }
            UpdateEvent::ProxyTestLatencyDone => {
                self.proxy_tree.end_testing();
            }
        }
        Ok(None)
    }

    fn handle_input(&mut self, event: InputEvent) -> Result<Option<Action>> {
        match event {
            InputEvent::TabGoto(index) => {
                if index >= 1 && index <= self.page_len() as u8 {
                    self.page_index = index - 1
                }
            }
            InputEvent::ToggleDebug => {
                self.show_debug = !self.show_debug;
                // On the debug page
                if self.page_index == Self::TITLES.len() as u8 - 1 {
                    self.page_index -= 1;
                } else if self.show_debug {
                    self.page_index = self.debug_page_index()
                }
            }
            InputEvent::Esc => {
                if let Some(mut list) = self.active_list() {
                    list.end();
                }
            }
            InputEvent::ToggleHold => {
                if let Some(mut list) = self.active_list() {
                    list.toggle();
                }
            }
            InputEvent::List(list_event) => {
                if let Some(mut list) = self.active_list() {
                    return Ok(list.handle(list_event));
                }
            }
            InputEvent::TestLatency => {
                if self.title() == "Proxies" && !self.proxy_tree.is_testing() {
                    self.proxy_tree.start_testing();
                    let group = self.proxy_tree.current_group();
                    let proxies = group
                        .members()
                        .iter()
                        .filter(|x| x.proxy_type().is_normal())
                        .map(|x| x.name().into())
                        .collect();
                    return Ok(Some(Action::TestLatency { proxies }));
                }
            }
            InputEvent::NextSort => {
                if let Some(mut list) = self.active_list() {
                    list.next_sort();
                }
            }
            InputEvent::PrevSort => {
                if let Some(mut list) = self.active_list() {
                    list.prev_sort();
                }
            }
            InputEvent::Other(_) => {} // InterfaceEvent::Other(event) => self.handle_list(event),
        }
        Ok(None)
    }

    pub const fn debug_page_index(&self) -> u8 {
        Self::TITLES.len() as u8 - 1
    }

    fn drop_events(&mut self, num: usize) -> impl Iterator<Item = Event> + '_ {
        self.debug_state.drain(..num)
    }
}
