use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};

use crate::{app::App, formatting::format_currency, models::TrackerMode};

use super::theme::{self, ThemePalette};

pub fn render_inspector(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let lines = match app.tracker_mode() {
        TrackerMode::Expense => expense_detail_lines(app, palette),
        TrackerMode::Saving => saving_detail_lines(app, palette),
        TrackerMode::Earning => earning_detail_lines(app, palette),
    };

    let detail = Paragraph::new(lines)
        .style(Style::default().fg(palette.text))
        .block(theme::panel_block(
            if en { "Inspector" } else { "Inspektor" },
            palette,
            palette.info,
            true,
        ))
        .wrap(Wrap { trim: true });
    frame.render_widget(detail, area);
}

fn expense_detail_lines(app: &App, palette: ThemePalette) -> Vec<Line<'static>> {
    let en = app.is_english();
    if let Some(item) = app.selected_expense() {
        let (signal, signal_color) = expense_signal(item.amount, palette);
        common_lines(
            app,
            palette,
            vec![
                Line::from(vec![
                    Span::styled(
                        if en {
                            " CASHFLOW SIGNAL "
                        } else {
                            " SINYAL CASHFLOW "
                        },
                        Style::default()
                            .fg(palette.text)
                            .bg(palette.accent_soft)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        format!(" {signal} "),
                        Style::default()
                            .fg(Color::Black)
                            .bg(signal_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::styled(
                    if en {
                        format!(
                            "expense {} | saving {} | net {}",
                            format_currency(app.total_expense_amount()),
                            format_currency(app.total_saving_amount()),
                            format_currency(app.net_balance())
                        )
                    } else {
                        format!(
                            "keluar {} | tabung {} | net {}",
                            format_currency(app.total_expense_amount()),
                            format_currency(app.total_saving_amount()),
                            format_currency(app.net_balance())
                        )
                    },
                    Style::default().fg(palette.muted),
                ),
                Line::raw(""),
                theme::key_value_line(
                    palette,
                    if en { "Mode" } else { "Mode" },
                    if en {
                        "Expense Board"
                    } else {
                        "Board Pengeluaran"
                    },
                    palette.accent,
                ),
                theme::key_value_line(
                    palette,
                    if en { "Signal" } else { "Sinyal" },
                    signal,
                    signal_color,
                ),
                theme::key_value_line(
                    palette,
                    if en { "Date" } else { "Tanggal" },
                    &item.date,
                    palette.text,
                ),
                theme::key_value_line(
                    palette,
                    if en { "Category" } else { "Kategori" },
                    item.category.label(app.language_preset()),
                    palette.text,
                ),
                theme::key_value_line(
                    palette,
                    if en { "Description" } else { "Deskripsi" },
                    &item.description,
                    palette.text,
                ),
                theme::key_value_line(
                    palette,
                    if en { "Amount" } else { "Nominal" },
                    &format_currency(item.amount),
                    palette.warn,
                ),
            ],
        )
    } else {
        empty_lines(
            app,
            if en {
                "No expense data yet."
            } else {
                "Belum ada pengeluaran."
            },
            if en {
                "Press 'a' to add a new entry."
            } else {
                "Tekan 'a' untuk tambah data baru."
            },
            palette,
        )
    }
}

fn saving_detail_lines(app: &App, palette: ThemePalette) -> Vec<Line<'static>> {
    let en = app.is_english();
    if let Some(item) = app.selected_saving() {
        let (signal, signal_color) = saving_signal(item.amount, palette);
        common_lines(
            app,
            palette,
            vec![
                Line::from(vec![
                    Span::styled(
                        if en {
                            " SAVING SIGNAL "
                        } else {
                            " SINYAL TABUNGAN "
                        },
                        Style::default()
                            .fg(palette.text)
                            .bg(palette.accent_soft)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        format!(" {signal} "),
                        Style::default()
                            .fg(Color::Black)
                            .bg(signal_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::styled(
                    if en {
                        format!(
                            "saving {} | expense {} | net {}",
                            format_currency(app.total_saving_amount()),
                            format_currency(app.total_expense_amount()),
                            format_currency(app.net_balance())
                        )
                    } else {
                        format!(
                            "tabung {} | keluar {} | net {}",
                            format_currency(app.total_saving_amount()),
                            format_currency(app.total_expense_amount()),
                            format_currency(app.net_balance())
                        )
                    },
                    Style::default().fg(palette.muted),
                ),
                Line::raw(""),
                theme::key_value_line(
                    palette,
                    if en { "Mode" } else { "Mode" },
                    if en { "Saving Board" } else { "Board Nabung" },
                    palette.accent,
                ),
                theme::key_value_line(
                    palette,
                    if en { "Signal" } else { "Sinyal" },
                    signal,
                    signal_color,
                ),
                theme::key_value_line(
                    palette,
                    if en { "Date" } else { "Tanggal" },
                    &item.date,
                    palette.text,
                ),
                theme::key_value_line(
                    palette,
                    if en { "Note" } else { "Catatan" },
                    &item.description,
                    palette.text,
                ),
                theme::key_value_line(
                    palette,
                    if en { "Amount" } else { "Nominal" },
                    &format_currency(item.amount),
                    palette.warn,
                ),
            ],
        )
    } else {
        empty_lines(
            app,
            if en {
                "No saving data yet."
            } else {
                "Belum ada data nabung."
            },
            if en {
                "Switch to Saving mode and press 'a'."
            } else {
                "Pilih mode Nabung lalu tekan 'a'."
            },
            palette,
        )
    }
}

fn earning_detail_lines(app: &App, palette: ThemePalette) -> Vec<Line<'static>> {
    let en = app.is_english();
    if let Some(item) = app.selected_earning() {
        let (signal, signal_color) = earning_signal(item.amount, palette);
        common_lines(
            app,
            palette,
            vec![
                Line::from(vec![
                    Span::styled(
                        if en {
                            " EARNING SIGNAL "
                        } else {
                            " SINYAL PEMASUKAN "
                        },
                        Style::default()
                            .fg(palette.text)
                            .bg(palette.accent_soft)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        format!(" {signal} "),
                        Style::default()
                            .fg(Color::Black)
                            .bg(signal_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::styled(
                    if en {
                        format!(
                            "earning {} | saving {} | net {}",
                            format_currency(app.total_earning_amount()),
                            format_currency(app.total_saving_amount()),
                            format_currency(app.net_balance())
                        )
                    } else {
                        format!(
                            "masuk {} | tabung {} | net {}",
                            format_currency(app.total_earning_amount()),
                            format_currency(app.total_saving_amount()),
                            format_currency(app.net_balance())
                        )
                    },
                    Style::default().fg(palette.muted),
                ),
                Line::raw(""),
                theme::key_value_line(
                    palette,
                    if en { "Mode" } else { "Mode" },
                    if en {
                        "Earning Board"
                    } else {
                        "Board Pemasukan"
                    },
                    palette.accent,
                ),
                theme::key_value_line(
                    palette,
                    if en { "Signal" } else { "Sinyal" },
                    signal,
                    signal_color,
                ),
                theme::key_value_line(
                    palette,
                    if en { "Date" } else { "Tanggal" },
                    &item.date,
                    palette.text,
                ),
                theme::key_value_line(
                    palette,
                    if en { "Source" } else { "Sumber" },
                    &item.description,
                    palette.text,
                ),
                theme::key_value_line(
                    palette,
                    if en { "Amount" } else { "Nominal" },
                    &format_currency(item.amount),
                    palette.info,
                ),
            ],
        )
    } else {
        empty_lines(
            app,
            if en {
                "No earning data yet."
            } else {
                "Belum ada data pemasukan."
            },
            if en {
                "Switch to Earning mode and press 'a'."
            } else {
                "Pilih mode Pemasukan lalu tekan 'a'."
            },
            palette,
        )
    }
}

fn common_lines(
    app: &App,
    palette: ThemePalette,
    mut lines: Vec<Line<'static>>,
) -> Vec<Line<'static>> {
    let en = app.is_english();
    lines.extend([
        Line::raw(""),
        theme::key_value_line(
            palette,
            if en { "Selection" } else { "Pilihan" },
            match app.tracker_mode() {
                TrackerMode::Expense => {
                    if app.selected_expense().is_some() {
                        if en { "expense live" } else { "expense aktif" }
                    } else {
                        "-"
                    }
                }
                TrackerMode::Saving => {
                    if app.selected_saving().is_some() {
                        if en { "saving live" } else { "nabung aktif" }
                    } else {
                        "-"
                    }
                }
                TrackerMode::Earning => {
                    if app.selected_earning().is_some() {
                        if en {
                            "earning live"
                        } else {
                            "pemasukan aktif"
                        }
                    } else {
                        "-"
                    }
                }
            },
            palette.info,
        ),
        theme::key_value_line(
            palette,
            if en { "Filter" } else { "Filter" },
            if app.filter_is_active() {
                app.filter()
            } else {
                "-"
            },
            palette.muted,
        ),
        theme::key_value_line(
            palette,
            "Savings",
            &format_currency(app.total_saving_amount()),
            palette.info,
        ),
        theme::key_value_line(
            palette,
            if en { "Earnings" } else { "Pemasukan" },
            &format_currency(app.total_earning_amount()),
            palette.accent,
        ),
        theme::key_value_line(
            palette,
            "Balance",
            &format_currency(app.net_balance()),
            palette.warn,
        ),
        theme::key_value_line(
            palette,
            if en { "Targets" } else { "Targets" },
            &format!(
                "{} {}",
                app.targets().len(),
                if en { "item" } else { "item" }
            ),
            palette.info,
        ),
        theme::key_value_line(
            palette,
            if en { "Target Page" } else { "Halaman Target" },
            &format!("{}/{}", app.target_page() + 1, app.target_page_count()),
            palette.info,
        ),
        Line::raw(""),
        Line::styled(
            if en {
                "Actions: 1/2/3 mode  |  a add flow  |  b balance  |  g target"
            } else {
                "Aksi: 1/2/3 mode  |  a tambah arus  |  b balance  |  g target"
            },
            Style::default().fg(palette.muted),
        ),
        Line::styled(
            if en {
                "Search: / filter  |  c clear filter  |  p target page  |  t theme"
            } else {
                "Cari: / filter  |  c clear filter  |  p target page  |  t theme"
            },
            Style::default().fg(palette.muted),
        ),
    ]);
    lines
}

fn empty_lines(app: &App, message: &str, hint: &str, palette: ThemePalette) -> Vec<Line<'static>> {
    let en = app.is_english();
    vec![
        Line::styled(message.to_string(), Style::default().fg(palette.muted)),
        Line::styled(hint.to_string(), Style::default().fg(palette.text)),
        Line::raw(""),
        Line::styled(
            if en {
                "finance board will light up after the first entry"
            } else {
                "board finansial akan hidup setelah entry pertama"
            },
            Style::default().fg(palette.warn),
        ),
        Line::raw(""),
        theme::key_value_line(
            palette,
            if en { "Targets" } else { "Targets" },
            &format!(
                "{} {}",
                app.targets().len(),
                if en { "item" } else { "item" }
            ),
            palette.info,
        ),
        theme::key_value_line(
            palette,
            if en { "Target Page" } else { "Halaman Target" },
            &format!("{}/{}", app.target_page() + 1, app.target_page_count()),
            palette.info,
        ),
    ]
}

fn expense_signal(amount: f64, palette: ThemePalette) -> (&'static str, Color) {
    if amount >= 1_000_000.0 {
        ("HEAVY", palette.danger)
    } else if amount >= 250_000.0 {
        ("MEDIUM", palette.warn)
    } else {
        ("LIGHT", palette.info)
    }
}

fn saving_signal(amount: f64, palette: ThemePalette) -> (&'static str, Color) {
    if amount >= 1_000_000.0 {
        ("BOOST", palette.accent)
    } else if amount >= 250_000.0 {
        ("BUILD", palette.info)
    } else {
        ("SEED", palette.warn)
    }
}

fn earning_signal(amount: f64, palette: ThemePalette) -> (&'static str, Color) {
    if amount >= 2_000_000.0 {
        ("SURGE", palette.accent)
    } else if amount >= 500_000.0 {
        ("FLOW", palette.info)
    } else {
        ("SPARK", palette.warn)
    }
}
