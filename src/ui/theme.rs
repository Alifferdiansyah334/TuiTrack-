use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::state::ThemePreset;

#[derive(Clone, Copy)]
pub struct ThemePalette {
    pub bg: Color,
    pub surface: Color,
    pub text: Color,
    pub muted: Color,
    pub accent: Color,
    pub accent_soft: Color,
    pub warn: Color,
    pub danger: Color,
    pub info: Color,
}

pub fn palette(preset: ThemePreset) -> ThemePalette {
    match preset {
        ThemePreset::Forest => ThemePalette {
            bg: Color::Rgb(8, 14, 10),
            surface: Color::Rgb(14, 24, 18),
            text: Color::Rgb(236, 244, 238),
            muted: Color::Rgb(103, 124, 109),
            accent: Color::Rgb(92, 208, 118),
            accent_soft: Color::Rgb(29, 64, 36),
            warn: Color::Rgb(233, 196, 82),
            danger: Color::Rgb(222, 100, 96),
            info: Color::Rgb(89, 198, 214),
        },
        ThemePreset::Amber => ThemePalette {
            bg: Color::Rgb(24, 14, 8),
            surface: Color::Rgb(35, 23, 14),
            text: Color::Rgb(247, 238, 225),
            muted: Color::Rgb(155, 130, 106),
            accent: Color::Rgb(255, 180, 56),
            accent_soft: Color::Rgb(79, 47, 11),
            warn: Color::Rgb(255, 214, 102),
            danger: Color::Rgb(229, 117, 87),
            info: Color::Rgb(255, 144, 81),
        },
        ThemePreset::Ocean => ThemePalette {
            bg: Color::Rgb(7, 13, 22),
            surface: Color::Rgb(13, 24, 37),
            text: Color::Rgb(228, 240, 247),
            muted: Color::Rgb(110, 132, 150),
            accent: Color::Rgb(76, 185, 255),
            accent_soft: Color::Rgb(21, 58, 88),
            warn: Color::Rgb(115, 220, 214),
            danger: Color::Rgb(255, 116, 155),
            info: Color::Rgb(118, 201, 255),
        },
        ThemePreset::Mono => ThemePalette {
            bg: Color::Rgb(12, 12, 12),
            surface: Color::Rgb(22, 22, 22),
            text: Color::Rgb(236, 236, 236),
            muted: Color::Rgb(138, 138, 138),
            accent: Color::Rgb(224, 224, 224),
            accent_soft: Color::Rgb(58, 58, 58),
            warn: Color::Rgb(198, 198, 198),
            danger: Color::Rgb(170, 170, 170),
            info: Color::Rgb(208, 208, 208),
        },
    }
}

pub fn render_background(frame: &mut Frame, theme: ThemePalette) {
    let bg = Paragraph::new("")
        .style(Style::default().bg(theme.bg))
        .block(Block::default().style(Style::default().bg(theme.bg)));
    frame.render_widget(bg, frame.area());
}

pub fn panel_block(title: &str, theme: ThemePalette, color: Color, active: bool) -> Block<'static> {
    let border = if active { color } else { theme.muted };
    let title_style = if active {
        Style::default().fg(theme.text).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.muted)
    };

    Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED)
        .style(Style::default().bg(theme.surface).fg(theme.text))
        .border_style(Style::default().fg(border))
        .title(Span::styled(format!(" {title} "), title_style))
}

pub fn modal_block(title: &str, theme: ThemePalette) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED)
        .style(Style::default().bg(theme.surface).fg(theme.text))
        .border_style(Style::default().fg(theme.accent))
        .title(Span::styled(
            format!(" {title} "),
            Style::default().fg(theme.text).add_modifier(Modifier::BOLD),
        ))
}

pub fn render_metric_card(
    frame: &mut Frame,
    area: Rect,
    theme: ThemePalette,
    title: &str,
    value: &str,
    subtitle: &str,
    color: Color,
) {
    let block = panel_block(title, theme, color, true);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let parts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .margin(1)
        .split(inner);

    frame.render_widget(
        Paragraph::new(value.to_string())
            .alignment(Alignment::Center)
            .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
        parts[0],
    );
    frame.render_widget(
        Paragraph::new(subtitle.to_string())
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme.muted)),
        parts[1],
    );
    frame.render_widget(
        Paragraph::new(Line::from(" "))
            .style(Style::default().bg(color))
            .block(Block::default().style(Style::default().bg(color))),
        parts[2],
    );
}

pub fn tab_chip(theme: ThemePalette, label: &str, active: bool) -> Span<'static> {
    if active {
        Span::styled(
            format!(" {label} "),
            Style::default()
                .fg(Color::Black)
                .bg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(
            format!(" {label} "),
            Style::default().fg(theme.muted).bg(theme.surface),
        )
    }
}

pub fn status_chip(theme: ThemePalette, label: &str, active: bool) -> Span<'static> {
    let color = if active { theme.warn } else { theme.muted };
    Span::styled(
        format!(" {label} "),
        Style::default().fg(Color::Black).bg(color),
    )
}

pub fn footer_chip(theme: ThemePalette, label: &str) -> Span<'static> {
    Span::styled(
        format!(" {label} "),
        Style::default().fg(theme.text).bg(theme.accent_soft),
    )
}

pub fn key_value_line(
    theme: ThemePalette,
    label: &str,
    value: &str,
    color: Color,
) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("{label}: "),
            Style::default()
                .fg(theme.muted)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(value.to_string(), Style::default().fg(color)),
    ])
}
