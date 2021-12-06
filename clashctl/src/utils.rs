use clap_generate::Shell;
use env_logger::fmt::Color;
use env_logger::Builder;
use log::{Level, LevelFilter};
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

pub fn init_logger(level: Option<LevelFilter>) {
    let mut builder = Builder::new();

    if let Some(lf) = level {
        builder.filter_level(lf);
    } else if let Ok(s) = ::std::env::var("CLASHCTL_LOG") {
        builder.parse_filters(&s);
    } else {
        builder.filter_level(LevelFilter::Info);
    }

    builder.format(|f, record| {
        use std::io::Write;
        let mut style = f.style();

        let level = match record.level() {
            Level::Trace => style.set_color(Color::Magenta).value("Trace"),
            Level::Debug => style.set_color(Color::Blue).value("Debug"),
            Level::Info => style.set_color(Color::Green).value(" Info"),
            Level::Warn => style.set_color(Color::Yellow).value(" Warn"),
            Level::Error => style.set_color(Color::Red).value("Error"),
        };

        writeln!(f, " {} > {}", level, record.args(),)
    });

    builder.init()
}
