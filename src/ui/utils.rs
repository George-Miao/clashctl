use std::{
    collections::VecDeque,
    ops::Range,
    sync::{mpsc::Sender, Mutex},
    thread::sleep,
    time::{Duration, Instant},
};

use log::{LevelFilter, Record};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders},
};

use crate::{
    model::{self, Log},
    DiagnosticEvent, Event, Result,
};

pub struct Interval {
    interval: Duration,
    deadline: Option<Instant>,
}

impl Interval {
    pub fn every(interval: Duration) -> Self {
        Self {
            interval,
            deadline: None,
        }
    }

    pub fn next_tick(&mut self) -> Duration {
        let now = Instant::now();
        if self.deadline.is_none() {
            self.deadline = Some(now + self.interval)
        }
        let deadline = self.deadline.unwrap();
        if now > deadline {
            let mut point = deadline;
            loop {
                point += self.interval;
                if point > now {
                    break point - now;
                }
            }
        } else {
            deadline - now
        }
    }

    pub fn tick(&mut self) {
        sleep(self.next_tick())
    }
}

#[test]
fn test_interval() {
    let mut interval = Interval::every(Duration::from_millis(100));
    assert!(interval.next_tick().as_millis().abs_diff(100) < 2);
    sleep(Duration::from_millis(50));
    assert!(interval.next_tick().as_millis().abs_diff(50) < 2);
}

pub struct Pulse {
    pulse: u64,
    counter: u64,
}

impl Pulse {
    #[inline]
    pub fn new(pulse: u64) -> Self {
        Self { pulse, counter: 0 }
    }

    #[inline]
    pub fn tick(&mut self) -> bool {
        let ret = self.is_pulse();
        self.counter += 1;
        ret
    }

    #[inline]
    pub fn is_pulse(&self) -> bool {
        self.counter % self.pulse == 0
    }
}

impl From<model::Level> for Color {
    fn from(val: model::Level) -> Self {
        match val {
            model::Level::Debug => Color::Gray,
            model::Level::Info => Color::Blue,
            model::Level::Warning => Color::Yellow,
            model::Level::Error => Color::Red,
        }
    }
}

pub struct TicksCounter {
    ticks: u64,
    time: Instant,
    inner: VecDeque<u64>,
}

impl Default for TicksCounter {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            ticks: Default::default(),
            inner: Default::default(),
        }
    }
}

impl TicksCounter {
    pub fn new_with_time(time: Instant) -> Self {
        Self {
            time,
            ..Self::default()
        }
    }

    pub fn new_tick(&mut self) {
        self.ticks += 1;
        self.inner.push_front(
            Instant::now()
                .duration_since(self.time)
                .as_millis()
                .try_into()
                .expect(
                    "Hey anyone who sees this as a panic message. Is the universe still there?",
                ),
        );
        if self.inner.len() > 100 {
            self.inner.drain(50..);
        }
    }

    pub fn tick_rate(&self) -> Option<f64> {
        // Ticks per Second
        Some(20_000.0 / ((self.inner.get(0)? - self.inner.get(20)?) as f64))
    }

    pub fn tick_num(&self) -> u64 {
        self.ticks
    }
}

pub struct Logger {
    sender: Mutex<Sender<Event>>,
    level: LevelFilter,
}

impl Logger {
    pub fn new(sender: Sender<Event>) -> Self {
        Self::new_with_level(sender, LevelFilter::Info)
    }

    pub fn new_with_level(sender: Sender<Event>, level: LevelFilter) -> Self {
        Self {
            sender: Mutex::new(sender),
            level,
        }
    }

    pub fn apply(self) -> Result<()> {
        let level = self.level;
        Ok(log::set_boxed_logger(Box::new(self)).map(|()| log::set_max_level(level))?)
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        let text = format!(
            "{:<6}{:?}",
            record.level().to_string().to_uppercase(),
            record.args()
        );
        self.sender
            .lock()
            .unwrap()
            .send(Event::Diagnostic(DiagnosticEvent::Log(text)))
            .unwrap()
    }
    fn flush(&self) {}
}

#[macro_export]
macro_rules! define_widget {
    ($name:ident) => {
        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub(crate) struct $name<'a> {
            state: &'a $crate::ui::TuiStates<'a>,
            _life: ::std::marker::PhantomData<&'a ()>,
        }

        impl<'a> $name<'a> {
            pub(crate) fn new(state: &'a $crate::ui::TuiStates<'a>) -> Self {
                Self {
                    _life: ::std::marker::PhantomData,
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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
    pub hold: bool,
}

impl Coord {
    pub fn toggle(&mut self) {
        if self.hold {
            *self = Self::default()
        } else {
            self.hold = true
        }
    }
}
// pub trait HumanReadable {
//     fn to_human_readable(&self) -> Option<String>;
// }

// const SECS_PER_MINUTE: i64 = 59;
// const SECS_PER_HOUR: i64 = 3599;
// const SECS_PER_DAY: i64 = 86399;
// const SECS_PER_WEEK: i64 = 604799;

// impl HumanReadable for Duration {
//     fn to_human_readable(&self) -> Option<String> {
//         match self.num_seconds() {
//             i64::MIN..0 => None,
//             num @ 0..SECS_PER_MINUTE => {
//                 Some(format!("{} sec{}", num, if num != 1 { "s" } else { "" }))
//             }
//             num @ SECS_PER_MINUTE..SECS_PER_HOUR => {
//                 let val = num / 60;
//                 Some(format!("{} min{}", val, if val != 1 { "s" } else { "" }))
//             }
//             num @ SECS_PER_HOUR..SECS_PER_DAY => {
//                 let val = num / 3600;
//                 Some(format!("{} hour{}", val, if val != 1 { "s" } else { "" }))
//             }
//             num @ SECS_PER_DAY..SECS_PER_WEEK => {
//                 let val = num / 86400;
//                 Some(format!("{} day{}", val, if val != 1 { "s" } else { "" }))
//             }
//             num => {
//                 let val = num / 604800;
//                 Some(format!("{} week{}", val, if val != 1 { "s" } else { "" }))
//             }
//         }
//     }
// }
