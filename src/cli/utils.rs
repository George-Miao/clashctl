use clap_generate::Shell;
use env_logger::fmt::Color;
use env_logger::Builder;
use log::Level;
use std::{env, path::PathBuf};

pub fn detect_shell() -> Option<Shell> {
    match env::var("SHELL") {
        Ok(shell) => PathBuf::from(shell)
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| name.parse().ok()),
        Err(_) => None,
    }
}

pub fn init_logger() {
    let mut builder = Builder::new();

    if let Ok(s) = ::std::env::var("CLASHCTL_LOG") {
        builder.parse_filters(&s);
    } else {
        builder.parse_filters("INFO");
    }

    builder.format(|f, record| {
        use std::io::Write;
        let mut style = f.style();

        let level = match record.level() {
            Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
            Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
            Level::Info => style.set_color(Color::Green).value("INFO "),
            Level::Warn => style.set_color(Color::Yellow).value("WARN "),
            Level::Error => style.set_color(Color::Red).value("ERROR"),
        };

        writeln!(f, " {} > {}", level, record.args(),)
    });

    builder.init()
}
