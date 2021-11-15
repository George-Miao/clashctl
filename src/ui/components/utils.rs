use std::ops::Range;

use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders},
};

use crate::model::Log;

#[macro_export]
macro_rules! define_widget {
    ($name:ident) => {
        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub(crate) struct $name<'a> {
            state: &'a $crate::ui::TuiStates<'a>,
            _life: PhantomData<&'a ()>,
        }

        impl<'a> $name<'a> {
            pub(crate) fn new(state: &'a $crate::ui::TuiStates<'a>) -> Self {
                Self {
                    _life: PhantomData,
                    state,
                }
            }
        }
    };
}

pub struct StyledChar {
    content: char,
    style: Style,
}

pub trait IntoSpans {
    fn into_spans<'a>(self) -> Spans<'a>;
}

impl IntoSpans for Vec<StyledChar> {
    fn into_spans<'a>(self) -> Spans<'a> {
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

pub fn string_window(string: &str, range: &Range<usize>) -> String {
    string.chars().skip(range.start).take(range.end).collect()
}

pub fn spans_window<'a, 'b>(spans: &'a Spans, range: &Range<usize>) -> Spans<'b> {
    let (start, end) = (range.start, range.end);
    let mut ret = Vec::with_capacity(spans.width());
    for Span { content, style } in &spans.0 {
        content.chars().for_each(|c| {
            ret.push(StyledChar {
                content: c,
                style: *style,
            })
        })
    }
    ret.into_iter()
        .skip(start)
        .take(end - start)
        .collect::<Vec<_>>()
        .into_spans()
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

impl<'a> From<Log> for Spans<'a> {
    fn from(val: Log) -> Self {
        let color = val.log_type.clone().into();
        Spans::from(vec![
            Span::styled(
                format!(" {:<5}", val.log_type.to_string().to_uppercase()),
                Style::default().fg(color),
            ),
            Span::raw(" "),
            Span::raw(val.payload),
        ])
    }
}

impl<'a> From<&Log> for Spans<'a> {
    fn from(val: &Log) -> Self {
        let color = val.log_type.clone().into();
        Spans::from(vec![
            Span::styled(
                format!(" {:<5}", val.log_type.to_string().to_uppercase()),
                Style::default().fg(color),
            ),
            Span::raw(" "),
            Span::raw(val.payload.to_owned()),
        ])
    }
}

#[test]
fn test_into_span() {
    let style_blue = Style::default().fg(Color::Blue);
    let style_plain = Style::default();
    let style_red = Style::default().fg(Color::Red);

    let chars_blue = "Hello".chars().map(|c| StyledChar {
        content: c,
        style: style_blue,
    });
    let chars_plain = StyledChar {
        content: ' ',
        style: style_plain,
    };
    let chars_red = "World 中文测试".chars().map(|c| StyledChar {
        content: c,
        style: style_red,
    });
    let spans = chars_blue
        .chain(std::iter::once(chars_plain))
        .chain(chars_red)
        .collect::<Vec<_>>()
        .into_spans();

    assert_eq!(
        spans,
        Spans::from(vec![
            Span {
                content: "Hello".into(),
                style: style_blue
            },
            Span {
                content: " ".into(),
                style: style_plain
            },
            Span {
                content: "World 中文测试".into(),
                style: style_red
            },
        ])
    )
}

pub trait GenericStatefulWidget<T> {
    type State;
    fn render(self, area: Rect, buf: &mut Buffer, state: &Self::State);
}

pub trait GenericWidget<T> {
    fn render(self, area: Rect, buf: &mut Buffer);
}
