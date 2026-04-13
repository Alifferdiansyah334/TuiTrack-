use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Cell, Row, Table, TableState},
};

use crate::{
    app::App,
    formatting::format_currency,
    models::{ExpenseCategory, TrackerMode},
    state::PanelFocus,
};

use super::theme::{self, ThemePalette};

pub fn render_table(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    match app.tracker_mode() {
        TrackerMode::Expense => render_expense_table(frame, area, app, palette),
        TrackerMode::Saving => render_saving_table(frame, area, app, palette),
        TrackerMode::Earning => render_earning_table(frame, area, app, palette),
    }
}

fn render_expense_table(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let indices = app.filtered_expense_indices();
    let total_amount: f64 = indices
        .iter()
        .filter_map(|idx| app.expense_at(*idx))
        .map(|item| item.amount)
        .sum();
    let rows = indices
        .iter()
        .enumerate()
        .filter_map(|(row_idx, idx)| app.expense_at(*idx).map(|item| (row_idx, item)))
        .map(|(row_idx, item)| {
            let (category_color, category_short) = expense_category_style(item.category, palette);
            let row_bg = if row_idx % 2 == 0 {
                palette.surface
            } else {
                palette.accent_soft
            };
            Row::new(vec![
                Cell::from(Line::from(vec![badge(category_short, category_color)])),
                Cell::from(item.date.clone()),
                Cell::from(Line::from(vec![Span::styled(
                    item.category.label(app.language_preset()).to_string(),
                    Style::default()
                        .fg(category_color)
                        .add_modifier(Modifier::BOLD),
                )])),
                Cell::from(Line::from(vec![Span::styled(
                    item.description.clone(),
                    Style::default().fg(palette.text),
                )])),
                Cell::from(Line::from(vec![Span::styled(
                    format_currency(item.amount),
                    Style::default()
                        .fg(palette.danger)
                        .add_modifier(Modifier::BOLD),
                )])),
            ])
            .style(Style::default().bg(row_bg))
        });

    let title = if app.filter_is_active() {
        format!(
            "{} ({})  Total: {}  {}: {}",
            if en {
                "Cash Out Queue"
            } else {
                "Antrian Arus Keluar"
            },
            indices.len(),
            format_currency(total_amount),
            if en { "Filter" } else { "Filter" },
            app.filter()
        )
    } else {
        format!(
            "{} ({})  Total: {}",
            if en {
                "Cash Out Queue"
            } else {
                "Antrian Arus Keluar"
            },
            indices.len(),
            format_currency(total_amount)
        )
    };

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Min(18),
            Constraint::Length(14),
        ],
    )
    .header(
        Row::new(vec![
            if en { "Lane" } else { "Jalur" },
            if en { "Date" } else { "Tanggal" },
            if en { "Category" } else { "Kategori" },
            if en { "Description" } else { "Deskripsi" },
            if en { "Amount" } else { "Nominal" },
        ])
        .style(
            Style::default()
                .fg(palette.text)
                .bg(palette.accent_soft)
                .add_modifier(Modifier::BOLD),
        ),
    )
    .block(theme::panel_block(
        &title,
        palette,
        palette.accent,
        app.panel_focus() == PanelFocus::Table && app.tracker_mode() == TrackerMode::Expense,
    ))
    .row_highlight_style(
        Style::default()
            .bg(palette.accent_soft)
            .fg(palette.text)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ");

    let mut state = TableState::default();
    if !indices.is_empty() {
        state.select(Some(app.expense_selected_index()));
    }
    frame.render_stateful_widget(table, area, &mut state);
}

fn render_saving_table(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let indices = app.filtered_saving_indices();
    let total_amount: f64 = indices
        .iter()
        .filter_map(|idx| app.saving_at(*idx))
        .map(|item| item.amount)
        .sum();
    let rows = indices
        .iter()
        .enumerate()
        .filter_map(|(row_idx, idx)| app.saving_at(*idx).map(|item| (row_idx, item)))
        .map(|(row_idx, item)| {
            let row_bg = if row_idx % 2 == 0 {
                palette.surface
            } else {
                palette.accent_soft
            };
            Row::new(vec![
                Cell::from(Line::from(vec![badge(
                    if en { "SAVE" } else { "NABUNG" },
                    palette.accent,
                )])),
                Cell::from(item.date.clone()),
                Cell::from(Line::from(vec![Span::styled(
                    item.description.clone(),
                    Style::default().fg(palette.text),
                )])),
                Cell::from(Line::from(vec![Span::styled(
                    format_currency(item.amount),
                    Style::default()
                        .fg(palette.accent)
                        .add_modifier(Modifier::BOLD),
                )])),
            ])
            .style(Style::default().bg(row_bg))
        });

    let title = if app.filter_is_active() {
        format!(
            "{} ({})  Total: {}  {}: {}",
            if en { "Saving List" } else { "Daftar Nabung" },
            indices.len(),
            format_currency(total_amount),
            if en { "Filter" } else { "Filter" },
            app.filter()
        )
    } else {
        format!(
            "{} ({})  Total: {}",
            if en { "Saving List" } else { "Daftar Nabung" },
            indices.len(),
            format_currency(total_amount)
        )
    };

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Min(24),
            Constraint::Length(14),
        ],
    )
    .header(
        Row::new(vec![
            if en { "Lane" } else { "Jalur" },
            if en { "Date" } else { "Tanggal" },
            if en { "Note" } else { "Catatan" },
            if en { "Amount" } else { "Nominal" },
        ])
        .style(
            Style::default()
                .fg(palette.text)
                .bg(palette.accent_soft)
                .add_modifier(Modifier::BOLD),
        ),
    )
    .block(theme::panel_block(
        &title,
        palette,
        palette.accent,
        app.panel_focus() == PanelFocus::Table && app.tracker_mode() == TrackerMode::Saving,
    ))
    .row_highlight_style(
        Style::default()
            .bg(palette.accent_soft)
            .fg(palette.text)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ");

    let mut state = TableState::default();
    if !indices.is_empty() {
        state.select(Some(app.saving_selected_index()));
    }
    frame.render_stateful_widget(table, area, &mut state);
}

fn render_earning_table(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let indices = app.filtered_earning_indices();
    let total_amount: f64 = indices
        .iter()
        .filter_map(|idx| app.earning_at(*idx))
        .map(|item| item.amount)
        .sum();
    let rows = indices
        .iter()
        .enumerate()
        .filter_map(|(row_idx, idx)| app.earning_at(*idx).map(|item| (row_idx, item)))
        .map(|(row_idx, item)| {
            let row_bg = if row_idx % 2 == 0 {
                palette.surface
            } else {
                palette.accent_soft
            };
            Row::new(vec![
                Cell::from(Line::from(vec![badge(
                    if en { "EARN" } else { "MASUK" },
                    palette.info,
                )])),
                Cell::from(item.date.clone()),
                Cell::from(Line::from(vec![Span::styled(
                    item.description.clone(),
                    Style::default().fg(palette.text),
                )])),
                Cell::from(Line::from(vec![Span::styled(
                    format_currency(item.amount),
                    Style::default()
                        .fg(palette.info)
                        .add_modifier(Modifier::BOLD),
                )])),
            ])
            .style(Style::default().bg(row_bg))
        });

    let title = if app.filter_is_active() {
        format!(
            "{} ({})  Total: {}  {}: {}",
            if en {
                "Income Queue"
            } else {
                "Antrian Pemasukan"
            },
            indices.len(),
            format_currency(total_amount),
            if en { "Filter" } else { "Filter" },
            app.filter()
        )
    } else {
        format!(
            "{} ({})  Total: {}",
            if en {
                "Income Queue"
            } else {
                "Antrian Pemasukan"
            },
            indices.len(),
            format_currency(total_amount)
        )
    };

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Min(24),
            Constraint::Length(14),
        ],
    )
    .header(
        Row::new(vec![
            if en { "Lane" } else { "Jalur" },
            if en { "Date" } else { "Tanggal" },
            if en { "Source" } else { "Sumber" },
            if en { "Amount" } else { "Nominal" },
        ])
        .style(
            Style::default()
                .fg(palette.text)
                .bg(palette.accent_soft)
                .add_modifier(Modifier::BOLD),
        ),
    )
    .block(theme::panel_block(
        &title,
        palette,
        palette.info,
        app.panel_focus() == PanelFocus::Table && app.tracker_mode() == TrackerMode::Earning,
    ))
    .row_highlight_style(
        Style::default()
            .bg(palette.accent_soft)
            .fg(palette.text)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ");

    let mut state = TableState::default();
    if !indices.is_empty() {
        state.select(Some(app.earning_selected_index()));
    }
    frame.render_stateful_widget(table, area, &mut state);
}

fn expense_category_style(
    category: ExpenseCategory,
    palette: ThemePalette,
) -> (Color, &'static str) {
    match category {
        ExpenseCategory::Primer => (palette.danger, "PRIME"),
        ExpenseCategory::Sekunder => (palette.warn, "MID"),
        ExpenseCategory::Tersier => (palette.info, "FLEX"),
    }
}

fn badge(label: &str, color: Color) -> Span<'static> {
    Span::styled(
        format!(" {label} "),
        Style::default()
            .fg(Color::Black)
            .bg(color)
            .add_modifier(Modifier::BOLD),
    )
}
