use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Bar, BarChart, BarGroup, Paragraph},
};

use chrono::{Local, Timelike};

use crate::{
    app::App,
    formatting::{center_horizontally, short_date_label},
    state::PanelFocus,
};

use super::theme::{self, ThemePalette};

pub fn render_deadline_radar(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    render_bucket_chart(
        frame,
        area,
        app,
        palette,
        if app.is_english() {
            "Deadline Radar"
        } else {
            "Radar Deadline"
        },
        if app.is_english() {
            "red / yellow / green pressure"
        } else {
            "tekanan merah / kuning / hijau"
        },
        app.work_urgency_buckets(app.is_english()),
        app.panel_focus() == PanelFocus::ExpenseChart,
        palette.danger,
    );
}

pub fn render_task_status(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    render_bucket_chart(
        frame,
        area,
        app,
        palette,
        if app.is_english() {
            "Task Status"
        } else {
            "Status Tugas"
        },
        if app.is_english() {
            "pending vs done split"
        } else {
            "komposisi belum vs selesai"
        },
        app.work_status_buckets(app.is_english()),
        app.panel_focus() == PanelFocus::SavingChart,
        palette.info,
    );
}

pub fn render_live_clock(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let now = Local::now();
    let hour = now.hour();
    let minute = now.minute();
    let second = now.second();
    let millisecond = now.nanosecond() / 1_000_000;
    let separator = if app.animation_tick() % 2 == 0 {
        ":"
    } else {
        " "
    };
    let digital = format!("{hour:02}{separator}{minute:02}:{second:02}.{millisecond:03}");
    let pulse = match app.animation_tick() % 4 {
        0 => "[==      ]",
        1 => "[====    ]",
        2 => "[======  ]",
        _ => "[========]",
    };
    let sync_chip = if en { "SYNC LIVE" } else { "SYNC LIVE" };
    let zone_chip = format!("UTC{}", now.format("%:z"));

    let block = theme::panel_block(
        if en { "Live Clock" } else { "Jam Sekarang" },
        palette,
        palette.info,
        true,
    );
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 4 || inner.height < 4 {
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Min(2),
        ])
        .margin(1)
        .split(inner);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            badge(sync_chip, palette.info),
            Span::raw(" "),
            Span::styled(
                now.format("%Y-%m-%d").to_string(),
                Style::default().fg(palette.muted),
            ),
            Span::raw(" "),
            badge(&zone_chip, palette.warn),
        ])),
        rows[0],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                pulse,
                Style::default()
                    .fg(palette.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                if en { "clock pulse" } else { "denyut jam" },
                Style::default().fg(palette.muted),
            ),
        ]))
        .alignment(Alignment::Center),
        rows[1],
    );

    frame.render_widget(
        Paragraph::new(digital).alignment(Alignment::Center).style(
            Style::default()
                .fg(palette.info)
                .add_modifier(Modifier::BOLD),
        ),
        rows[2],
    );

    frame.render_widget(
        Paragraph::new(if en {
            "hour : minute : second . millisecond"
        } else {
            "jam : menit : detik . milidetik"
        })
        .alignment(Alignment::Center)
        .style(Style::default().fg(palette.warn)),
        rows[3],
    );

    let breakdown = vec![
        Line::from(vec![
            metric_chip(
                if en { "HOUR" } else { "JAM" },
                format!("{hour:02}"),
                palette.accent,
            ),
            Span::raw(" "),
            metric_chip(
                if en { "MIN" } else { "MENIT" },
                format!("{minute:02}"),
                palette.warn,
            ),
        ]),
        Line::from(vec![
            metric_chip(
                if en { "SEC" } else { "DETIK" },
                format!("{second:02}"),
                palette.danger,
            ),
            Span::raw(" "),
            metric_chip(
                if en { "MS" } else { "MS" },
                format!("{millisecond:03}"),
                palette.info,
            ),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(breakdown)
            .alignment(Alignment::Center)
            .style(Style::default().fg(palette.text)),
        rows[4],
    );
}

pub fn render_performance(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    render_day_chart(
        frame,
        area,
        app,
        palette,
        if app.is_english() {
            "Performance"
        } else {
            "Performa"
        },
        app.work_completed_by_day(),
        if app.is_english() {
            "Completed tasks per day"
        } else {
            "Tugas selesai per hari"
        },
        app.panel_focus() == PanelFocus::TargetA,
        palette.accent,
    );
}

pub fn render_task_flow(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    render_day_chart(
        frame,
        area,
        app,
        palette,
        if app.is_english() {
            "Task Flow"
        } else {
            "Arus Tugas"
        },
        app.work_created_by_day(),
        if app.is_english() {
            "New tasks per day"
        } else {
            "Tugas baru per hari"
        },
        app.panel_focus() == PanelFocus::TargetB,
        palette.warn,
    );
}

fn render_bucket_chart(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    palette: ThemePalette,
    title: &str,
    subtitle: &str,
    data: Vec<(&'static str, u64)>,
    active: bool,
    accent: ratatui::style::Color,
) {
    let bars = data
        .iter()
        .enumerate()
        .map(|(idx, (label, value))| {
            Bar::default()
                .label(Line::from(*label))
                .value(app.animated_chart_value(*value as f64, idx))
                .text_value(value.to_string())
        })
        .collect::<Vec<_>>();
    render_chart_card(
        frame,
        area,
        app,
        title,
        subtitle.to_string(),
        bars,
        data.iter().map(|(_, v)| *v).max().unwrap_or(0),
        data.iter().map(|(_, v)| *v).sum(),
        active,
        palette,
        accent,
    );
}

fn render_day_chart(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    palette: ThemePalette,
    title: &str,
    data: Vec<(String, u64)>,
    subtitle: &str,
    active: bool,
    accent: ratatui::style::Color,
) {
    let bars = data
        .iter()
        .enumerate()
        .map(|(idx, (label, value))| {
            Bar::default()
                .label(Line::from(short_date_label(label)))
                .value(app.animated_chart_value(*value as f64, idx))
                .text_value(value.to_string())
        })
        .collect::<Vec<_>>();
    render_chart_card(
        frame,
        area,
        app,
        title,
        subtitle.to_string(),
        bars,
        data.iter().map(|(_, v)| *v).max().unwrap_or(0),
        data.iter().map(|(_, v)| *v).sum(),
        active,
        palette,
        accent,
    );
}

fn render_chart_card(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    title: &str,
    subtitle: String,
    bars: Vec<Bar<'static>>,
    max: u64,
    total: u64,
    active: bool,
    palette: ThemePalette,
    accent: ratatui::style::Color,
) {
    let block = theme::panel_block(title, palette, accent, active);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 4 || inner.height < 4 {
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(4),
        ])
        .margin(1)
        .split(inner);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            badge(animated_chart_tag(app, title), accent),
            Span::raw(" "),
            Span::styled(
                if app.is_english() {
                    format!("peak {}", max)
                } else {
                    format!("puncak {}", max)
                },
                Style::default().fg(palette.text),
            ),
            Span::raw("   "),
            Span::styled(
                if app.is_english() {
                    format!("total {}", total)
                } else {
                    format!("total {}", total)
                },
                Style::default().fg(palette.muted),
            ),
        ]))
        .alignment(Alignment::Center),
        rows[0],
    );
    frame.render_widget(
        Paragraph::new(subtitle)
            .alignment(Alignment::Center)
            .style(Style::default().fg(palette.warn)),
        rows[1],
    );

    if bars.is_empty() || max == 0 {
        frame.render_widget(
            Paragraph::new(if app.is_english() {
                "No signal yet"
            } else {
                "Belum ada sinyal"
            })
            .alignment(Alignment::Center)
            .style(Style::default().fg(palette.muted)),
            rows[2],
        );
        return;
    }

    let count = bars.len() as u16;
    let bar_gap = 1;
    let bar_width = if count <= 2 { 5 } else { 3 };
    let required_width = count
        .saturating_mul(bar_width)
        .saturating_add(count.saturating_sub(1).saturating_mul(bar_gap))
        .saturating_add(2);
    let chart_area = center_horizontally(rows[2], required_width.min(rows[2].width));

    frame.render_widget(
        BarChart::default()
            .direction(Direction::Vertical)
            .bar_width(bar_width)
            .bar_gap(bar_gap)
            .group_gap(0)
            .bar_style(Style::default().fg(accent))
            .value_style(
                Style::default()
                    .fg(ratatui::style::Color::Black)
                    .bg(accent)
                    .add_modifier(Modifier::BOLD),
            )
            .label_style(Style::default().fg(palette.text))
            .max(max.max(1))
            .data(BarGroup::default().bars(&bars)),
        chart_area,
    );
}

fn metric_chip(label: &str, value: String, color: ratatui::style::Color) -> Span<'static> {
    Span::styled(
        format!(" {label} {value} "),
        Style::default()
            .fg(ratatui::style::Color::Black)
            .bg(color)
            .add_modifier(Modifier::BOLD),
    )
}

fn badge(label: &str, color: ratatui::style::Color) -> Span<'static> {
    Span::styled(
        format!(" {label} "),
        Style::default()
            .fg(ratatui::style::Color::Black)
            .bg(color)
            .add_modifier(Modifier::BOLD),
    )
}

fn animated_chart_tag<'a>(app: &App, title: &'a str) -> &'a str {
    match title {
        "Deadline Radar" | "Radar Deadline" => match app.animation_tick() % 4 {
            0 => "RADAR SWEEP",
            1 => "SCAN LIVE",
            2 => "RISK MAP",
            _ => "SCAN LIVE",
        },
        "Task Status" | "Status Tugas" => match app.animation_tick() % 4 {
            0 => "QUEUE MIX",
            1 => "LOAD SPLIT",
            2 => "STATUS MIX",
            _ => "LOAD SPLIT",
        },
        "Performance" | "Performa" => match app.animation_tick() % 4 {
            0 => "WIN STREAK",
            1 => "DONE PULSE",
            2 => "OUTPUT RUN",
            _ => "DONE PULSE",
        },
        _ => match app.animation_tick() % 4 {
            0 => "FLOW LINE",
            1 => "INPUT WAVE",
            2 => "ENTRY RUN",
            _ => "INPUT WAVE",
        },
    }
}
