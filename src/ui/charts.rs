use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Bar, BarChart, BarGroup, Paragraph},
};

use crate::{
    app::App,
    formatting::{center_horizontally, compact_currency, format_currency, short_date_label},
    state::PanelFocus,
};

use super::theme::{self, ThemePalette};

pub fn render_charts(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[0]);
    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[1]);

    render_expense_chart(frame, top[0], app, palette);
    render_inflow_chart(frame, top[1], app, palette);
    let targets = app.visible_targets();
    render_target_chart(frame, bottom[0], app, palette, targets.first(), 0);
    render_target_chart(frame, bottom[1], app, palette, targets.get(1), 1);
}

fn render_inflow_chart(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    match app.tracker_mode() {
        crate::models::TrackerMode::Earning => render_earning_chart(frame, area, app, palette),
        _ => render_saving_chart(frame, area, app, palette),
    }
}

fn render_expense_chart(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let (top_category, top_total) = app.top_expense_category();
    let totals = app.expense_category_totals();
    let bars = totals
        .iter()
        .enumerate()
        .map(|(idx, (category, total))| {
            Bar::default()
                .label(Line::from(
                    category.label(app.language_preset()).to_string(),
                ))
                .value(app.animated_chart_value(*total, idx))
                .text_value(compact_currency(*total))
        })
        .collect::<Vec<_>>();
    let max = totals
        .iter()
        .map(|(_, total)| *total)
        .fold(0.0_f64, f64::max);

    render_chart_card(
        frame,
        area,
        app,
        if en {
            "Expense Chart"
        } else {
            "Chart Pengeluaran"
        },
        if en {
            "category pressure across outgoing flow"
        } else {
            "tekanan kategori pada arus keluar"
        },
        format!(
            "{}: {} ({})",
            if en { "Top" } else { "Top" },
            top_category.label(app.language_preset()),
            format_currency(top_total)
        ),
        bars,
        max,
        app.panel_focus() == PanelFocus::ExpenseChart,
        palette,
        palette.danger,
        if en {
            "No data yet."
        } else {
            "Belum ada data."
        },
        None,
    );
}

fn render_saving_chart(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let (top_day, top_total) = app.top_saving_day();
    let totals = app.saving_totals_by_day();
    let max = totals
        .iter()
        .map(|(_, total)| *total)
        .fold(0.0_f64, f64::max);
    let bars = totals
        .iter()
        .take(5)
        .enumerate()
        .map(|(idx, (date, total))| {
            Bar::default()
                .label(Line::from(short_date_label(date)))
                .value(app.animated_chart_value(*total, idx))
                .text_value(compact_currency(*total))
        })
        .collect::<Vec<_>>();

    render_chart_card(
        frame,
        area,
        app,
        if en { "Saving Chart" } else { "Chart Nabung" },
        if en {
            "daily saving rhythm"
        } else {
            "ritme tabungan harian"
        },
        format!(
            "{}: {} ({})",
            if en { "Top" } else { "Top" },
            top_day,
            format_currency(top_total)
        ),
        bars,
        max,
        app.panel_focus() == PanelFocus::SavingChart,
        palette,
        palette.accent,
        if en {
            "No data yet."
        } else {
            "Belum ada data."
        },
        None,
    );
}

fn render_earning_chart(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let (top_day, top_total) = app.top_earning_day();
    let totals = app.earning_totals_by_day();
    let max = totals
        .iter()
        .map(|(_, total)| *total)
        .fold(0.0_f64, f64::max);
    let bars = totals
        .iter()
        .take(5)
        .enumerate()
        .map(|(idx, (date, total))| {
            Bar::default()
                .label(Line::from(short_date_label(date)))
                .value(app.animated_chart_value(*total, idx))
                .text_value(compact_currency(*total))
        })
        .collect::<Vec<_>>();

    render_chart_card(
        frame,
        area,
        app,
        if en {
            "Earning Chart"
        } else {
            "Chart Pemasukan"
        },
        if en {
            "daily income rhythm"
        } else {
            "ritme pemasukan harian"
        },
        format!(
            "{}: {} ({})",
            if en { "Top" } else { "Top" },
            top_day,
            format_currency(top_total)
        ),
        bars,
        max,
        app.panel_focus() == PanelFocus::SavingChart,
        palette,
        palette.info,
        if en {
            "No data yet."
        } else {
            "Belum ada data."
        },
        None,
    );
}

fn render_target_chart(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    palette: ThemePalette,
    target: Option<&crate::state::BudgetTarget>,
    slot: usize,
) {
    let en = app.is_english();
    let accent = if slot == 0 {
        palette.info
    } else {
        palette.warn
    };

    let Some(target) = target else {
        let page_text = format!(
            "{} {}/{}",
            if en { "Page" } else { "Halaman" },
            app.target_page() + 1,
            app.target_page_count()
        );
        render_chart_card(
            frame,
            area,
            app,
            if slot == 0 {
                if en {
                    "Target Window A"
                } else {
                    "Jendela Target A"
                }
            } else if en {
                "Target Window B"
            } else {
                "Jendela Target B"
            },
            if en {
                "target slot waiting for a new mission"
            } else {
                "slot target menunggu misi baru"
            },
            page_text,
            Vec::new(),
            0.0,
            app.panel_focus()
                == if slot == 0 {
                    PanelFocus::TargetA
                } else {
                    PanelFocus::TargetB
                },
            palette,
            accent,
            if en {
                "No target in this slot."
            } else {
                "Belum ada target di slot ini."
            },
            Some(if en {
                "g add target  |  p page"
            } else {
                "g tambah target  |  p page"
            }),
        );
        return;
    };

    let (current, achieved, remaining, percent) = app.target_progress(target);
    let subtitle = if target.amount > 0.0 {
        format!(
            "{}  |  {:.0}%  |  {}/{}",
            target.mode.label(app.language_preset()),
            percent,
            format_currency(current.max(0.0)),
            format_currency(target.amount)
        )
    } else if en {
        "Set a target with key g".into()
    } else {
        "Set target dengan tombol g".into()
    };

    let bars = if target.amount > 0.0 {
        vec![
            Bar::default()
                .label(Line::from(if en { "Now" } else { "Kini" }))
                .value(app.animated_chart_value(achieved, 0))
                .text_value(compact_currency(achieved)),
            Bar::default()
                .label(Line::from(if en { "Left" } else { "Sisa" }))
                .value(app.animated_chart_value(remaining, 1))
                .text_value(compact_currency(remaining)),
        ]
    } else {
        Vec::new()
    };

    render_chart_card(
        frame,
        area,
        app,
        &target.title,
        if en {
            "progress window"
        } else {
            "jendela progres"
        },
        subtitle,
        bars,
        target.amount,
        app.panel_focus()
            == if slot == 0 {
                PanelFocus::TargetA
            } else {
                PanelFocus::TargetB
            },
        palette,
        accent,
        if en {
            "No target yet."
        } else {
            "Belum ada target."
        },
        Some(if en {
            "Enter delete target  |  p page"
        } else {
            "Enter hapus target  |  p page"
        }),
    );
}

fn render_chart_card(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    title: &str,
    descriptor: &str,
    summary: String,
    bars: Vec<Bar<'static>>,
    max_value: f64,
    active: bool,
    palette: ThemePalette,
    accent: ratatui::style::Color,
    empty_label: &str,
    hint: Option<&str>,
) {
    let block = theme::panel_block(title, palette, accent, active);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 4 || inner.height < 4 {
        return;
    }

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(4),
            Constraint::Length(if hint.is_some() { 1 } else { 0 }),
        ])
        .margin(1)
        .split(inner);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            chart_badge(chart_tag(app, title), accent),
            Span::raw(" "),
            Span::styled(descriptor.to_string(), Style::default().fg(palette.warn)),
        ]))
        .alignment(Alignment::Center),
        sections[0],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(summary, Style::default().fg(palette.text)),
            Span::raw("   "),
            Span::styled(
                if app.is_english() {
                    format!("peak {}", format_currency(max_value.max(0.0)))
                } else {
                    format!("puncak {}", format_currency(max_value.max(0.0)))
                },
                Style::default().fg(palette.muted),
            ),
        ]))
        .alignment(Alignment::Center),
        sections[1],
    );

    if bars.is_empty() || max_value <= 0.0 {
        frame.render_widget(
            Paragraph::new(empty_label)
                .alignment(Alignment::Center)
                .style(Style::default().fg(palette.muted)),
            sections[2],
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
    let chart_area = center_horizontally(sections[2], required_width.min(sections[2].width));

    let chart = BarChart::default()
        .direction(ratatui::layout::Direction::Vertical)
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
        .max(max_value.ceil().max(1.0) as u64)
        .data(BarGroup::default().bars(&bars));

    frame.render_widget(chart, chart_area);

    if let Some(hint) = hint {
        frame.render_widget(
            Paragraph::new(hint)
                .alignment(Alignment::Center)
                .style(Style::default().fg(if active { accent } else { palette.muted })),
            sections[3],
        );
    }
}

fn chart_badge(label: &str, color: ratatui::style::Color) -> Span<'static> {
    Span::styled(
        format!(" {label} "),
        Style::default()
            .fg(ratatui::style::Color::Black)
            .bg(color)
            .add_modifier(Modifier::BOLD),
    )
}

fn chart_tag<'a>(app: &App, title: &'a str) -> &'a str {
    match title {
        "Expense Chart" | "Chart Pengeluaran" => match app.animation_tick() % 4 {
            0 => "CASH OUT",
            1 => "SPEND MAP",
            2 => "BURN RATE",
            _ => "SPEND MAP",
        },
        "Saving Chart" | "Chart Nabung" => match app.animation_tick() % 4 {
            0 => "SAVE PULSE",
            1 => "BUILD FLOW",
            2 => "GROWTH RUN",
            _ => "BUILD FLOW",
        },
        "Earning Chart" | "Chart Pemasukan" => match app.animation_tick() % 4 {
            0 => "EARN WAVE",
            1 => "INCOME RUN",
            2 => "FLOW BOOST",
            _ => "INCOME RUN",
        },
        _ => match app.animation_tick() % 4 {
            0 => "TARGET VIEW",
            1 => "GOAL TRACK",
            2 => "PROGRESS MAP",
            _ => "GOAL TRACK",
        },
    }
}
