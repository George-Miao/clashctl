mod config;
mod connection;
mod debug;
mod log;
mod proxy;
mod rule;
mod status;

use crate::ui::{Backend, TuiStates};

use tui::{layout::Rect, Frame};

pub fn route(state: &TuiStates, area: Rect, f: &mut Frame<Backend>) {
    match state.page_index {
        0 => f.render_widget(status::StatusPage::new(state), area),
        1 => f.render_widget(proxy::ProxyPage::new(state), area),
        2 => f.render_widget(rule::RulePage::new(state), area),
        3 => f.render_widget(connection::ConnectionPage::new(state), area),
        4 => f.render_widget(log::LogPage::new(state), area),
        5 => f.render_widget(config::ConfigPage::new(state), area),
        6 => f.render_widget(debug::DebugPage::new(state), area),
        _ => unreachable!(),
    };
}
