use std::{borrow::Cow, ops::Range};

use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders},
};

use crate::{IntoSpans, Wrap};

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

pub fn tagged_footer<T: ToString>(label: &str, style: Style, content: T) -> Spans {
    let mut ret = help_footer(label, style, style.add_modifier(Modifier::BOLD)).wrapped();
    ret.0.push(Span::styled(
        content.to_string().wrapped(),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::REVERSED),
    ));
    ret
}

pub fn string_window<'a>(string: &'a str, range: &Range<usize>) -> Cow<'a, str> {
    string
        .chars()
        .skip(range.start)
        .take(range.end - range.start)
        .collect()
}

pub fn string_window_owned(string: String, range: &Range<usize>) -> String {
    string
        .chars()
        .skip(range.start)
        .take(range.end - range.start)
        .collect()
}

pub fn spans_window<'a>(spans: &'a Spans, range: &Range<usize>) -> Spans<'a> {
    let inner = &spans.0;
    match inner.len() {
        0 => spans.to_owned(),
        1 => {
            let item = &inner[0];
            Spans(vec![Span::styled(
                string_window(&item.content, range),
                item.style,
            )])
        }
        _ => {
            let (start, end) = (range.start, range.end);
            inner
                .iter()
                .flat_map(|x| x.styled_graphemes(Style::default()))
                .skip(start)
                .take(end - start)
                .collect::<Vec<_>>()
                .into_spans()
        }
    }
}

pub fn spans_window_owned<'a>(mut spans: Spans<'a>, range: &Range<usize>) -> Spans<'a> {
    match spans.0.len() {
        0 => spans,
        1 => {
            let item = &mut spans.0[0];
            item.content = string_window_owned(item.content.to_string(), range).into();
            spans
        }
        _ => {
            let (start, end) = (range.start, range.end);
            spans
                .0
                .iter_mut()
                .flat_map(|x| x.content.chars().map(|c| (x.style, c)))
                .skip(start)
                .take(end - start)
                .collect::<Vec<_>>()
                .into_spans()
        }
    }
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

#[test]
fn test_string_window() {
    let test = "▼ 代理相关的 API".to_owned();
    assert_eq!("代理", &string_window(&test, &(2..4)));
    assert_eq!("理相关的 API", &string_window(&test, &(3..114)));
}
