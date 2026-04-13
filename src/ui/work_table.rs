use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Cell, Row, Table, TableState},
};

use crate::{app::App, models::WorkUrgency, state::PanelFocus};

use super::theme::{self, ThemePalette};

pub fn render_table(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let indices = app.filtered_work_indices();
    let rows = indices
        .iter()
        .enumerate()
        .filter_map(|(row_idx, idx)| app.work_at(*idx).map(|item| (row_idx, item)))
        .map(|(row_idx, item)| {
            let (accent, lane_label) = lane_style(item, palette, en);
            let row_bg = if row_idx % 2 == 0 {
                palette.surface
            } else {
                palette.accent_soft
            };
            let status_style = if item.completed {
                Style::default()
                    .fg(Color::Black)
                    .bg(palette.accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::Black)
                    .bg(accent)
                    .add_modifier(Modifier::BOLD)
            };

            Row::new(vec![
                Cell::from(Line::from(vec![Span::styled(
                    format!(" {lane_label} "),
                    Style::default()
                        .fg(Color::Black)
                        .bg(accent)
                        .add_modifier(Modifier::BOLD),
                )])),
                Cell::from(item.deadline.clone()),
                Cell::from(item.deadline_time.clone()),
                Cell::from(Line::from(vec![Span::styled(
                    countdown_label(item, en),
                    Style::default().fg(accent).add_modifier(Modifier::BOLD),
                )])),
                Cell::from(Line::from(vec![Span::styled(
                    format!(" {} ", item.status_label(app.language_preset())),
                    status_style,
                )])),
                Cell::from(Line::from(vec![Span::styled(
                    item.description.clone(),
                    Style::default()
                        .fg(palette.text)
                        .add_modifier(if item.completed {
                            Modifier::CROSSED_OUT
                        } else {
                            Modifier::empty()
                        }),
                )])),
            ])
            .style(Style::default().fg(palette.text).bg(row_bg))
        });

    let title = if app.filter_is_active() {
        format!(
            "{} ({})  Pending: {}  Done: {}  Filter: {}",
            if en { "Mission Queue" } else { "Antrian Misi" },
            indices.len(),
            app.work_pending_count(),
            app.work_completed_count(),
            app.filter()
        )
    } else {
        format!(
            "{} ({})  Pending: {}  Done: {}",
            if en { "Mission Queue" } else { "Antrian Misi" },
            indices.len(),
            app.work_pending_count(),
            app.work_completed_count()
        )
    };

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Length(8),
            Constraint::Length(12),
            Constraint::Length(16),
            Constraint::Min(18),
        ],
    )
    .header(
        Row::new(vec![
            if en { "Lane" } else { "Jalur" },
            if en { "Deadline" } else { "Deadline" },
            if en { "Time" } else { "Jam" },
            if en { "Clock" } else { "Hitung" },
            if en { "Status" } else { "Status" },
            if en { "Description" } else { "Deskripsi" },
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
        app.panel_focus() == PanelFocus::Table,
    ))
    .row_highlight_style(
        Style::default()
            .bg(palette.accent_soft)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ");

    let mut state = TableState::default();
    if !indices.is_empty() {
        state.select(Some(app.work_selected_index()));
    }
    frame.render_stateful_widget(table, area, &mut state);
}

fn lane_style(
    task: &crate::models::WorkTask,
    palette: ThemePalette,
    en: bool,
) -> (Color, &'static str) {
    if task.completed {
        return (palette.info, if en { "DONE" } else { "DONE" });
    }

    match task.urgency() {
        WorkUrgency::Red => (palette.danger, if en { "HOT" } else { "PANAS" }),
        WorkUrgency::Yellow => (Color::Yellow, if en { "NEXT" } else { "DEKAT" }),
        WorkUrgency::Green => (palette.accent, if en { "FLOW" } else { "AMAN" }),
        WorkUrgency::Done => (palette.info, if en { "DONE" } else { "DONE" }),
        WorkUrgency::Unknown => (palette.muted, if en { "CHECK" } else { "CEK" }),
    }
}

fn countdown_label(task: &crate::models::WorkTask, en: bool) -> String {
    match task.hours_until_deadline() {
        Some(hours) if hours < 0 => {
            if en {
                "overdue".into()
            } else {
                "terlambat".into()
            }
        }
        Some(hours) if hours < 24 => format!("{hours}h"),
        Some(hours) => format!("{}d", hours / 24),
        None => {
            if en {
                "unknown".into()
            } else {
                "invalid".into()
            }
        }
    }
}
