use tui::style::Color;

use crate::clashctl::model;

pub trait AsColor {
    fn as_color(&self) -> Color;
}

impl AsColor for model::Level {
    fn as_color(&self) -> Color {
        match self {
            model::Level::Debug => Color::Gray,
            model::Level::Info => Color::Blue,
            model::Level::Warning => Color::Yellow,
            model::Level::Error => Color::Red,
            _ => Color::Gray,
        }
    }
}

impl AsColor for log::Level {
    fn as_color(&self) -> Color {
        match self {
            log::Level::Debug => Color::Gray,
            log::Level::Info => Color::Blue,
            log::Level::Warning => Color::Yellow,
            log::Level::Error => Color::Red,
            _ => Color::Gray,
        }
    }
}
