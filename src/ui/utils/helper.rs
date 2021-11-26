use tui::{
    style::Style,
    text::{Span, Spans},
};

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
