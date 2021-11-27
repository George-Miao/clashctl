use std::{
    collections::VecDeque,
    fmt::Write,
    ops::Range,
    sync::{mpsc::Sender, Mutex},
    thread::sleep,
    time::{Duration, Instant},
};

use log::{LevelFilter, Record};
use tui::{
    style::{Color, Style},
    text::{Span, Spans, StyledGrapheme},
    widgets::{Block, Borders},
};

use crate::{
    model,
    ui::{DiagnosticEvent, Event},
    Result,
};

mod helper;

pub use helper::*;

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

pub trait AsColor {
    fn as_color(&self) -> Color;
}

impl AsColor for model::Level {
    fn as_color(&self) -> Color {
        match self {
            model::Level::Debug => Color::Gray,
            model::Level::Info => Color::Blue,
            model::Level::Warning => Color::Yellow,
            model::Level::Error => Color::Red,
        }
    }
}

impl AsColor for log::Level {
    fn as_color(&self) -> Color {
        match self {
            log::Level::Debug => Color::Gray,
            log::Level::Info => Color::Blue,
            log::Level::Warn => Color::Yellow,
            log::Level::Error => Color::Red,
            _ => Color::Gray,
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
        Ok(log::set_boxed_logger(Box::new(self)).map(|_| log::set_max_level(level))?)
    }
}

impl log::Log for Logger {
    fn enabled(&self, meta: &log::Metadata) -> bool {
        meta.level() <= self.level
    }
    fn log(&self, record: &Record) {
        self.sender
            .lock()
            .unwrap()
            .send(Event::Diagnostic(DiagnosticEvent::Log(
                record.level(),
                format!("{}", record.args()),
            )))
            .unwrap()
    }
    fn flush(&self) {}
}

#[macro_export]
macro_rules! define_widget {
    ($name:ident) => {
        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub struct $name<'a> {
            state: &'a $crate::ui::TuiStates<'a>,
            _life: ::std::marker::PhantomData<&'a ()>,
        }

        impl<'a> $name<'a> {
            pub fn new(state: &'a $crate::ui::TuiStates<'a>) -> Self {
                Self {
                    _life: ::std::marker::PhantomData,
                    state,
                }
            }
        }
    };
}

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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
    pub hold: bool,
}

impl Coord {
    pub fn toggle(&mut self) {
        if self.hold {
            self.end()
        } else {
            self.hold()
        }
    }

    pub fn end(&mut self) {
        *self = Self::default()
    }

    pub fn hold(&mut self) {
        self.hold = true
    }
}

pub trait HMS {
    fn as_second(&self) -> i64;
    fn hms(&self) -> String {
        let mut s = self.as_second();
        let mut neg = false;
        let mut written = false;
        if s < 0 {
            neg = true;
            s = -s;
        }
        let (h, s) = (s / 3600, s % 3600);
        let (m, s) = (s / 60, s % 60);
        let mut ret = String::with_capacity(10);
        if neg {
            written = true;
            ret.push('-')
        };
        if written || h > 0 {
            written = true;
            write!(ret, "{}h ", h).expect("Cannot write to buf")
        }
        if written || m > 0 {
            write!(ret, "{}m ", m).expect("Cannot write to buf")
        }
        write!(ret, "{}s", s).expect("Cannot write to buf");
        ret
    }
}

impl HMS for chrono::Duration {
    fn as_second(&self) -> i64 {
        self.num_seconds()
    }
}

impl HMS for Duration {
    fn as_second(&self) -> i64 {
        self.as_secs().try_into().expect("Seconds to big")
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
