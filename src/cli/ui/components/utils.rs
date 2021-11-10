use tui::{
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders},
};

pub fn get_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::LightBlue))
        .title(Span::styled(
            format!(" {} ", title),
            Style::default().fg(Color::Blue),
        ))
}

pub fn get_text_style() -> Style {
    Style::default().fg(Color::White)
}
