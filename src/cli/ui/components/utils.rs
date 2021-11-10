use tui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub fn get_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::LightBlue))
        .title(format!(" {} ", title))
}

pub fn get_focused_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Green))
        .title(format!(" {} ", title))
}

pub fn get_text_style() -> Style {
    Style::default().fg(Color::White)
}
