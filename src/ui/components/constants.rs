use std::borrow::Cow;

use tui::{
    style::{Color, Modifier, Style},
    text::Span,
};

pub struct Consts {}

impl Consts {
    pub const PROXY_LATENCY_SIGN: &'static str = "⬤ ";

    pub const NOT_PROXY_SIGN: &'static str = "✪ ";

    pub const NO_LATENCY_SIGN: &'static str = "⊝";

    pub const FOCUSED_INDICATOR: &'static str = "🮇  ";

    pub const FOCUSED_EXPANDED_INDICATOR: &'static str = "🮇   ";

    pub const UNFOCUSED_INDICATOR: &'static str = "   ";

    pub const EXPANDED_FOCUSED_INDICATOR: &'static str = "🮇  ➤";

    pub const DEFAULT_STYLE: Style = Style {
        add_modifier: Modifier::empty(),
        sub_modifier: Modifier::empty(),
        fg: None,
        bg: None,
    };

    pub const PROXY_TYPE_STYLE: Style = Style {
        fg: Some(Color::Gray),
        add_modifier: Modifier::DIM,
        ..Self::DEFAULT_STYLE
    };

    pub const NO_LATENCY_STYLE: Style = Style {
        fg: Some(Color::DarkGray),
        ..Self::DEFAULT_STYLE
    };

    pub const LOW_LATENCY_STYLE: Style = Style {
        fg: Some(Color::LightGreen),
        ..Self::DEFAULT_STYLE
    };

    pub const MID_LATENCY_STYLE: Style = Style {
        fg: Some(Color::LightYellow),
        ..Self::DEFAULT_STYLE
    };

    pub const HIGH_LATENCY_STYLE: Style = Style {
        fg: Some(Color::LightRed),
        ..Self::DEFAULT_STYLE
    };

    pub const DELIMITER_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(" "),
        style: Self::DEFAULT_STYLE,
    };

    pub const NOT_PROXY_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::NOT_PROXY_SIGN),
        style: Self::NO_LATENCY_STYLE,
    };

    pub const NO_LATENCY_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::PROXY_LATENCY_SIGN),
        style: Self::NO_LATENCY_STYLE,
    };

    pub const LOW_LATENCY_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::PROXY_LATENCY_SIGN),
        style: Self::LOW_LATENCY_STYLE,
    };

    pub const MID_LATENCY_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::PROXY_LATENCY_SIGN),
        style: Self::MID_LATENCY_STYLE,
    };

    pub const HIGH_LATENCY_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::PROXY_LATENCY_SIGN),
        style: Self::HIGH_LATENCY_STYLE,
    };

    pub const FOCUSED_INDICATOR_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::FOCUSED_INDICATOR),
        style: Style {
            fg: Some(Color::LightYellow),
            ..Self::DEFAULT_STYLE
        },
    };

    pub const UNFOCUSED_INDICATOR_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::UNFOCUSED_INDICATOR),
        style: Self::DEFAULT_STYLE,
    };

    pub const EXPANDED_INDICATOR_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::FOCUSED_EXPANDED_INDICATOR),
        style: Style {
            fg: Some(Color::LightYellow),
            ..Self::DEFAULT_STYLE
        },
    };

    pub const EXPANDED_FOCUSED_INDICATOR_SPAN: Span<'static> = Span {
        content: Cow::Borrowed(Self::EXPANDED_FOCUSED_INDICATOR),
        style: Style {
            fg: Some(Color::LightYellow),
            ..Self::DEFAULT_STYLE
        },
    };
}
