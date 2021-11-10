use std::ops::Range;

use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders},
};

pub struct StyledChar {
    content: char,
    style: Style,
}

pub trait IntoSpan {
    fn into_span<'a>(self) -> Spans<'a>;
}

impl IntoSpan for Vec<StyledChar> {
    fn into_span<'a>(self) -> Spans<'a> {
        self.group_by(|a, b| a.style == b.style)
            .map(|x| {
                let style = x
                    .get(0)
                    .expect("Should be at least one item in grouped slices")
                    .style;
                let content = x.iter().fold(String::new(), |mut acc, x| {
                    acc.push(x.content);
                    acc
                });
                Span {
                    content: content.into(),
                    style,
                }
            })
            .collect::<Vec<_>>()
            .into()
    }
}

pub fn spans_window(spans: Spans, range: Range<usize>) -> Spans {
    let (start, end) = (range.start, range.end);
    let mut ret = Vec::with_capacity(spans.width());
    for Span { content, style } in spans.0 {
        content
            .chars()
            .for_each(|c| ret.push(StyledChar { content: c, style }))
    }
    ret.into_iter()
        .skip(start)
        .take(end - start)
        .collect::<Vec<_>>()
        .into_span()
}

pub fn get_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::LightBlue))
        .title(Span::styled(
            format!(" {} ", title),
            Style::default().fg(Color::Blue),
        ))
}

pub fn get_focused_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            format!(" {} ", title),
            Style::default().fg(Color::LightGreen),
        ))
        .style(Style::default().fg(Color::Green))
}

pub fn get_text_style() -> Style {
    Style::default().fg(Color::White)
}

pub fn split(content: &str, index: usize) -> Option<(&str, &str)> {
    content
        .char_indices()
        .map(|(i, _)| i)
        .nth(index)
        .map(|x| content.split_at(x))
}

pub fn get_raw(content: &Spans) -> String {
    content
        .0
        .iter()
        .map(|x| &x.content)
        .fold(String::with_capacity(content.width() * 2), |acc, x| acc + x)
}
