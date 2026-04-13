use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::app::App;

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

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                if en { "Work Tracker" } else { "Work Tracker" },
                Style::default()
                    .fg(palette.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                if en {
                    "deadline board with live pressure, flow, and archive cues"
                } else {
                    "board deadline dengan tekanan live, arus, dan cue arsip"
                },
                Style::default().fg(palette.muted),
            ),
            Span::raw("   "),
            metric_pill(
                palette,
                if en {
                    format!("RED {}", app.work_red_zone_count())
                } else {
                    format!("MERAH {}", app.work_red_zone_count())
                },
                palette.danger,
            ),
            Span::raw(" "),
            metric_pill(
                palette,
                if en {
                    format!("DONE {}", app.work_completed_count())
                } else {
                    format!("SELESAI {}", app.work_completed_count())
                },
                palette.accent,
            ),
            Span::raw(" "),
            metric_pill(
                palette,
                if en {
                    format!("RATE {:.0}%", app.work_completion_rate())
                } else {
                    format!("RASIO {:.0}%", app.work_completion_rate())
                },
                palette.info,
            ),
        ])),
        rows[0],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            theme::tab_chip(
                palette,
                if en { "a Add" } else { "a Tambah" },
                app.mode() == crate::state::Mode::AddWork,
            ),
            Span::raw(" "),
            theme::tab_chip(palette, if en { "x Done" } else { "x Selesai" }, false),
            Span::raw(" "),
            theme::tab_chip(palette, if en { "d Delete" } else { "d Hapus" }, false),
            Span::raw(" "),
            theme::tab_chip(
                palette,
                if en { "/ Filter" } else { "/ Filter" },
                app.filter_is_active(),
            ),
            Span::raw(" "),
            theme::tab_chip(
                palette,
                "t Theme",
                app.mode() == crate::state::Mode::ThemeSelect,
            ),
            Span::raw(" "),
            theme::tab_chip(
                palette,
                if en { "l Language" } else { "l Bahasa" },
                app.mode() == crate::state::Mode::LanguageSelect,
            ),
            Span::raw("   "),
            theme::status_chip(
                palette,
                app.mode().label(app.language_preset()),
                app.mode() != crate::state::Mode::Normal,
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
                            "Urgency"
                        } else {
                            "Urgensi"
                        }
                    }
                    crate::state::PanelFocus::SavingChart => {
                        if en {
                            "Status"
                        } else {
                            "Status"
                        }
                    }
                    crate::state::PanelFocus::TargetA => {
                        if en {
                            "Performance"
                        } else {
                            "Performa"
                        }
                    }
                    crate::state::PanelFocus::TargetB => {
                        if en {
                            "Flow"
                        } else {
                            "Arus"
                        }
                    }
                },
                true,
            ),
        ])),
        rows[1],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("Hint: ", Style::default().fg(palette.muted)),
            Span::styled(app.mode_hint(), Style::default().fg(palette.text)),
            Span::raw("   "),
            Span::styled("Status: ", Style::default().fg(palette.muted)),
            Span::styled(app.status().to_string(), Style::default().fg(palette.warn)),
        ])),
        rows[2],
    );
}

fn metric_pill(_palette: ThemePalette, label: String, color: Color) -> Span<'static> {
    Span::styled(
        format!(" {label} "),
        Style::default()
            .fg(Color::Black)
            .bg(color)
            .add_modifier(Modifier::BOLD),
    )
}

pub fn render_summary(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let cards = [
        (
            if en { "Pending" } else { "Belum" },
            app.work_pending_count().to_string(),
            if en {
                "Unfinished tasks"
            } else {
                "Tugas belum selesai"
            },
            palette.warn,
        ),
        (
            if en { "Done" } else { "Selesai" },
            app.work_completed_count().to_string(),
            if en {
                "Completed tasks"
            } else {
                "Tugas selesai"
            },
            palette.accent,
        ),
        (
            if en { "Red Zone" } else { "Zona Merah" },
            app.work_red_zone_count().to_string(),
            if en {
                "Due today or overdue"
            } else {
                "Hari ini atau lewat deadline"
            },
            palette.danger,
        ),
        (
            if en { "H-1" } else { "H-1" },
            app.work_h1_count().to_string(),
            if en {
                "Within 24 hours"
            } else {
                "Dalam 24 jam"
            },
            Color::Yellow,
        ),
        (
            if en { "Rate" } else { "Rasio" },
            format!("{:.0}%", app.work_completion_rate()),
            if en {
                "Completion rate"
            } else {
                "Rasio selesai"
            },
            palette.info,
        ),
        (
            if en { "Focus" } else { "Fokus" },
            app.top_work_focus(),
            if en {
                "Nearest priority"
            } else {
                "Prioritas terdekat"
            },
            palette.info,
        ),
    ];

    let areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Ratio(1, cards.len() as u32); cards.len()])
        .split(area);

    for (idx, (title, value, subtitle, color)) in cards.into_iter().enumerate() {
        theme::render_metric_card(frame, areas[idx], palette, title, &value, subtitle, color);
    }
}
