use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{app::App, formatting::format_currency, models::TrackerMode, state::Mode};

use super::theme::{self, ThemePalette};

pub fn render_header(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let block = theme::panel_block("TuiTrack", palette, palette.accent, true);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .margin(1)
        .split(inner);

    let title = Line::from(vec![
        Span::styled(
            if en {
                "Expense + Saving + Earning Tracker"
            } else {
                "Tracker Pengeluaran + Nabung + Pemasukan"
            },
            Style::default()
                .fg(palette.text)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            if en {
                "finance board with cashflow, targets, and balance pulse"
            } else {
                "board finansial dengan arus kas, target, dan pulse balance"
            },
            Style::default().fg(palette.muted),
        ),
        Span::raw("   "),
        metric_pill(
            if en {
                format!("OUT {}", format_currency(app.total_expense_amount()))
            } else {
                format!("KELUAR {}", format_currency(app.total_expense_amount()))
            },
            palette.danger,
        ),
        Span::raw(" "),
        metric_pill(
            if en {
                format!("SAVE {}", format_currency(app.total_saving_amount()))
            } else {
                format!("TABUNG {}", format_currency(app.total_saving_amount()))
            },
            palette.accent,
        ),
        Span::raw(" "),
        metric_pill(
            if en {
                format!("EARN {}", format_currency(app.total_earning_amount()))
            } else {
                format!("MASUK {}", format_currency(app.total_earning_amount()))
            },
            palette.info,
        ),
        Span::raw(" "),
        metric_pill(
            format!("NET {}", format_currency(app.net_balance())),
            palette.warn,
        ),
    ]);
    frame.render_widget(Paragraph::new(title), rows[0]);

    let tabs = Line::from(vec![
        theme::tab_chip(
            palette,
            if en { "1 Expense" } else { "1 Pengeluaran" },
            app.tracker_mode() == TrackerMode::Expense,
        ),
        Span::raw(" "),
        theme::tab_chip(
            palette,
            if en { "2 Saving" } else { "2 Nabung" },
            app.tracker_mode() == TrackerMode::Saving,
        ),
        Span::raw(" "),
        theme::tab_chip(
            palette,
            if en { "3 Earning" } else { "3 Pemasukan" },
            app.tracker_mode() == TrackerMode::Earning,
        ),
        Span::raw(" "),
        theme::tab_chip(
            palette,
            if en { "a Add Flow" } else { "a Tambah Arus" },
            matches!(
                app.mode(),
                Mode::AddExpense | Mode::AddSaving | Mode::AddEarning
            ),
        ),
        Span::raw(" "),
        theme::tab_chip(palette, "b Balance", app.mode() == Mode::AddBalance),
        Span::raw(" "),
        theme::tab_chip(palette, "g Target", app.mode() == Mode::AddTarget),
        Span::raw(" "),
        theme::tab_chip(palette, "t Theme", app.mode() == Mode::ThemeSelect),
        Span::raw(" "),
        theme::tab_chip(
            palette,
            if en { "l Language" } else { "l Bahasa" },
            app.mode() == Mode::LanguageSelect,
        ),
        Span::raw("   "),
        theme::status_chip(
            palette,
            app.mode().label(app.language_preset()),
            app.mode() != Mode::Normal,
        ),
        Span::raw(" "),
        theme::status_chip(
            palette,
            if en { "Filter" } else { "Filter" },
            app.filter_is_active(),
        ),
        Span::raw(" "),
        theme::status_chip(
            palette,
            match app.panel_focus() {
                crate::state::PanelFocus::Table => {
                    if en {
                        "Table"
                    } else {
                        "Tabel"
                    }
                }
                crate::state::PanelFocus::ExpenseChart => {
                    if en {
                        "Expense Chart"
                    } else {
                        "Chart Expense"
                    }
                }
                crate::state::PanelFocus::SavingChart => match app.tracker_mode() {
                    TrackerMode::Earning => {
                        if en {
                            "Earning Chart"
                        } else {
                            "Chart Pemasukan"
                        }
                    }
                    _ => {
                        if en {
                            "Saving Chart"
                        } else {
                            "Chart Nabung"
                        }
                    }
                },
                crate::state::PanelFocus::TargetA => "Target A",
                crate::state::PanelFocus::TargetB => "Target B",
            },
            true,
        ),
    ]);
    frame.render_widget(Paragraph::new(tabs), rows[1]);

    let ribbon = Line::from(vec![
        Span::styled(
            if en { "Hint: " } else { "Hint: " },
            Style::default().fg(palette.muted),
        ),
        Span::styled(app.mode_hint(), Style::default().fg(palette.text)),
        Span::raw("   "),
        Span::styled(
            if en { "Status: " } else { "Status: " },
            Style::default().fg(palette.muted),
        ),
        Span::styled(app.status().to_string(), Style::default().fg(palette.warn)),
    ]);
    frame.render_widget(Paragraph::new(ribbon), rows[2]);
}

pub fn render_summary(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let cards_data = [
        (
            if en { "Expense" } else { "Expense" },
            format_currency(app.total_expense_amount()),
            if en {
                "Total outgoing flow"
            } else {
                "Arus keluar total"
            },
            palette.danger,
        ),
        (
            "Savings",
            format_currency(app.total_saving_amount()),
            if en {
                "From saving mode"
            } else {
                "Dari mode tabung"
            },
            palette.accent,
        ),
        (
            if en { "Earnings" } else { "Pemasukan" },
            format_currency(app.total_earning_amount()),
            if en {
                "From earning mode"
            } else {
                "Dari mode pemasukan"
            },
            palette.info,
        ),
        (
            if en { "Base Balance" } else { "Balance Dasar" },
            if app.balance_enabled() {
                format_currency(app.balance_amount())
            } else {
                "-".into()
            },
            if en {
                "Manual, optional"
            } else {
                "Manual, opsional"
            },
            palette.info,
        ),
        (
            "Balance",
            format_currency(app.net_balance()),
            if app.balance_enabled() {
                if en {
                    "Base + savings + earnings - expense"
                } else {
                    "Dasar + tabungan + pemasukan - expense"
                }
            } else {
                if en {
                    "Savings + earnings - expense"
                } else {
                    "Tabungan + pemasukan - expense"
                }
            },
            palette.warn,
        ),
        (
            if en { "This Month" } else { "Bulan Ini" },
            match app.tracker_mode() {
                TrackerMode::Expense => format_currency(app.monthly_expense_amount()),
                TrackerMode::Saving => format_currency(app.monthly_saving_amount()),
                TrackerMode::Earning => format_currency(app.monthly_earning_amount()),
            },
            if en {
                "Current period"
            } else {
                "Periode berjalan"
            },
            Color::Magenta,
        ),
        (
            "Insight",
            match app.tracker_mode() {
                TrackerMode::Expense => app
                    .top_expense_category()
                    .0
                    .label(app.language_preset())
                    .to_string(),
                TrackerMode::Saving => app.top_saving_day().0,
                TrackerMode::Earning => app.top_earning_day().0,
            },
            if en {
                "Active highlight"
            } else {
                "Highlight aktif"
            },
            palette.info,
        ),
    ];

    let cards = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Ratio(1, cards_data.len() as u32);
            cards_data.len()
        ])
        .split(area);

    for (idx, (title, value, subtitle, color)) in cards_data.into_iter().enumerate() {
        theme::render_metric_card(frame, cards[idx], palette, title, &value, subtitle, color);
    }
}

fn metric_pill(label: String, color: Color) -> Span<'static> {
    Span::styled(
        format!(" {label} "),
        Style::default()
            .fg(Color::Black)
            .bg(color)
            .add_modifier(Modifier::BOLD),
    )
}
