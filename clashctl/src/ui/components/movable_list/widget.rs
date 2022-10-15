use tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{List, ListItem, Widget},
};

use crate::{
    interactive::{EndlessSelf, Noop, SortMethod},
    spans_window_owned, tagged_footer,
    ui::{
        components::{
            Footer, FooterItem, FooterWidget, MovableListItem, MovableListManage, MovableListState,
        },
        utils::{get_block, get_focused_block, get_text_style},
    },
};

// TODO Fixed item on top
// Useful for table header
// Append to vec on each render
#[derive(Clone, Debug)]
pub struct MovableList<'a, T, S = Noop>
where
    T: MovableListItem<'a>,
    S: Default,
{
    pub(super) title: String,
    pub(super) state: &'a MovableListState<'a, T, S>,
}

impl<'a, T, S> MovableList<'a, T, S>
where
    S: SortMethod<T> + EndlessSelf + Default + ToString,
    T: MovableListItem<'a>,
    MovableListState<'a, T, S>: MovableListManage,
{
    pub fn new<TITLE: Into<String>>(title: TITLE, state: &'a MovableListState<'a, T, S>) -> Self {
        Self {
            state,
            title: title.into(),
        }
    }

    fn render_footer(&self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let mut footer = Footer::default();
        let pos = self.state.current_pos();

        let sort_str = self.state.sort.to_string();

        footer.push_right(FooterItem::span(Span::styled(
            format!(" Ln {}, Col {} ", pos.y, pos.x),
            Style::default()
                .fg(if pos.hold { Color::Green } else { Color::Blue })
                .add_modifier(Modifier::REVERSED),
        )));

        if pos.hold {
            let style = Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::REVERSED);

            footer.push_left(FooterItem::span(Span::styled(" FREE ", style)));
            footer.push_left(FooterItem::span(Span::styled(" [^] ▲ ▼ ◀ ▶ Move ", style)));
            if !sort_str.is_empty() {
                footer.push_left(tagged_footer("Sort", style, sort_str).into());
            }
        } else {
            let style = Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::REVERSED);

            footer.push_left(FooterItem::span(Span::styled(" NORMAL ", style)));
            footer.push_left(FooterItem::span(Span::styled(
                " SPACE / [^] ▲ ▼ ◀ ▶ Move ",
                style,
            )));
            if !sort_str.is_empty() {
                footer.push_left(tagged_footer("Sort", style, sort_str).into());
            }
        }

        let widget = FooterWidget::new(&footer);
        widget.render(area, buf);
    }
}

impl<'a, T, S> Widget for MovableList<'a, T, S>
where
    S: SortMethod<T> + EndlessSelf + Default + ToString,
    T: MovableListItem<'a>,
    MovableListState<'a, T, S>: MovableListManage,
{
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let num = self.state.items.len();

        let offset = self.state.offset;

        let block = if offset.hold {
            get_focused_block(&self.title)
        } else {
            get_block(&self.title)
        };
        let pad = self.state.padding;
        let inner = block.inner(area);
        let inner = if pad == 0 {
            inner
        } else {
            Rect {
                x: inner.x + pad,
                y: inner.y,
                width: inner.width.saturating_sub(pad * 2),
                height: inner.height,
            }
        };

        let height = inner.height as usize;

        // Calculate which portion of the list will be displayed
        let y_offset = if offset.y + 1 > num {
            num.saturating_sub(1)
        } else {
            offset.y
        };

        let x_offset = offset.x;

        let index_width = num.to_string().len();
        let index_style = Style::default().fg(Color::DarkGray);

        let x_range = x_offset
            ..(x_offset
                .saturating_add(inner.width as usize)
                .saturating_sub(index_width));
        let with_index = self.state.with_index;
        let rev_index = self.state.reverse_index;

        // Get that portion of items
        let items = if num != 0 {
            self.state
                .items
                .iter()
                .rev()
                .skip(y_offset)
                .take(height as usize)
                .enumerate()
                .map(|(i, x)| {
                    let content = x.to_spans();
                    let x_width = content.width();
                    let content = spans_window_owned(content, &x_range);

                    let mut spans = if x_width != 0 && content.width() == 0 {
                        Span::raw("◀").into()
                    } else {
                        content
                    };

                    if with_index {
                        let cur_index = if rev_index {
                            num - i - y_offset
                        } else {
                            i + y_offset + 1
                        };
                        spans.0.insert(
                            0,
                            Span::styled(
                                format!("{:>width$} ", cur_index, width = index_width),
                                index_style,
                            ),
                        );
                    };
                    ListItem::new(spans)
                })
                .collect::<Vec<_>>()
        } else {
            vec![ListItem::new(Span::raw(
                self.state
                    .placeholder
                    .to_owned()
                    .unwrap_or_else(|| "Nothing's here yet".into()),
            ))]
        };

        block.render(area, buf);
        List::new(items).style(get_text_style()).render(inner, buf);

        self.render_footer(area, buf);
    }
}

// #[test]
// fn test_movable_list() {
//     let items = &["Test1", "测试1", "[ABCD] 🇺🇲 测试 符号
// 106"].into_iter().map(|x| x.);     assert_eq!()
// }
