use std::ops::Range;

use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders},
};

use crate::ui::IntoSpans;

pub fn help_footer(content: &str, normal: Style, highlight: Style) -> Spans {
    if content.is_empty() {
        Spans(vec![])
    } else if content.len() == 1 {
        Spans(vec![Span::raw(content)])
    } else {
        let (index, _) = content.char_indices().nth(1).unwrap();
        let (first_char, rest) = content.split_at(index);
        Spans(vec![
            Span::styled("[", normal),
            Span::styled(first_char, highlight),
            Span::styled("]", normal),
            Span::styled(rest, normal),
        ])
    }
}

pub fn string_window(string: &str, range: &Range<usize>) -> String {
    string.chars().skip(range.start).take(range.end).collect()
}

pub fn spans_window<'a>(spans: &'a Spans, range: &Range<usize>) -> Spans<'a> {
    let (start, end) = (range.start, range.end);

    spans
        .0
        .iter()
        .flat_map(|x| x.styled_graphemes(Style::default()))
        .skip(start)
        .take(end - start)
        .into_spans()
}

pub fn get_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::LightBlue))
        .title(Span::raw(format!(" {} ", title)))
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
