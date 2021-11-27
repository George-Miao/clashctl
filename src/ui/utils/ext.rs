use tui::{
    style::{Color, Style},
    text::{Span, Spans, StyledGrapheme},
};

pub trait IntoSpan<'a> {
    fn into_span(self) -> Span<'a>;
}

impl<'a> IntoSpan<'a> for StyledGrapheme<'a> {
    fn into_span(self) -> Span<'a> {
        Span::styled(self.symbol, self.style)
    }
}

pub trait IntoSpans<'a> {
    fn into_spans(self) -> Spans<'a>;
}

impl<'a, T> IntoSpans<'a> for T
where
    T: Iterator<Item = StyledGrapheme<'a>>,
{
    fn into_spans(self) -> Spans<'a> {
        self.fold(None, |mut acc: Option<(Vec<Span<'a>>, Style)>, x| {
            let x_style = x.style;
            match acc {
                Some((ref mut vec, ref mut style)) => {
                    if style == &x_style {
                        vec.last_mut().expect("vec.len() >= 1").content += x.symbol;
                    } else {
                        vec.push(x.into_span());
                        *style = x_style
                    }
                }
                None => return Some((vec![x.into_span()], x_style)),
            };
            acc
        })
        .map(|x| x.0)
        .unwrap_or_default()
        .into()
    }
}

#[test]
fn test_into_span() {
    let style_blue = Style::default().fg(Color::Blue);
    let style_plain = Style::default();
    let style_red = Style::default().fg(Color::Red);

    let (a, b, c) = (
        Span::raw("Hello"),
        Span::raw(" "),
        Span::raw("World 中文测试"),
    );
    let chars_blue = a.styled_graphemes(style_blue);
    let chars_plain = b.styled_graphemes(style_plain);
    let chars_red = c.styled_graphemes(style_red);

    let spans = chars_blue.chain(chars_plain).chain(chars_red).into_spans();

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
