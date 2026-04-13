use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};

use crate::{app::App, models::WorkUrgency};

use super::theme::{self, ThemePalette};

pub fn render_inspector(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let lines = if let Some(task) = app.selected_work() {
        let (lane, lane_color, delete_fx) = match task.urgency() {
            WorkUrgency::Done => (
                if en { "Archive lane" } else { "Jalur arsip" },
                palette.accent,
                if en {
                    "archive burst on delete"
                } else {
                    "burst arsip saat dihapus"
                },
            ),
            WorkUrgency::Red => (
                if en { "Critical lane" } else { "Jalur kritis" },
                palette.danger,
                if en {
                    "hard drop on delete"
                } else {
                    "drop keras saat dihapus"
                },
            ),
            WorkUrgency::Yellow => (
                if en { "Watch lane" } else { "Jalur waspada" },
                palette.warn,
                if en {
                    "soft drop on delete"
                } else {
                    "drop cepat saat dihapus"
                },
            ),
            WorkUrgency::Green => (
                if en { "Flow lane" } else { "Jalur aman" },
                palette.info,
                if en {
                    "soft drop on delete"
                } else {
                    "drop cepat saat dihapus"
                },
            ),
            WorkUrgency::Unknown => (
                if en { "Check lane" } else { "Jalur cek" },
                palette.muted,
                if en {
                    "manual check before delete"
                } else {
                    "cek manual sebelum hapus"
                },
            ),
        };
        let queue_mix = if en {
            format!(
                "pending {} | done {} | red {}",
                app.work_pending_count(),
                app.work_completed_count(),
                app.work_red_zone_count()
            )
        } else {
            format!(
                "belum {} | selesai {} | merah {}",
                app.work_pending_count(),
                app.work_completed_count(),
                app.work_red_zone_count()
            )
        };

        vec![
            Line::from(vec![
                Span::styled(
                    if en {
                        " LIVE TASK SIGNAL "
                    } else {
                        " SINYAL TUGAS LIVE "
                    },
                    Style::default()
                        .fg(palette.text)
                        .bg(palette.accent_soft)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
                Span::styled(
                    format!(" {} ", lane),
                    Style::default()
                        .fg(Color::Black)
                        .bg(lane_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::styled(queue_mix, Style::default().fg(palette.muted)),
            Line::raw(""),
            theme::key_value_line(
                palette,
                if en { "Mode" } else { "Mode" },
                if en {
                    "Work Mission Board"
                } else {
                    "Board Misi Kerja"
                },
                palette.accent,
            ),
            theme::key_value_line(palette, if en { "Lane" } else { "Jalur" }, lane, lane_color),
            theme::key_value_line(
                palette,
                if en { "Date" } else { "Tanggal" },
                &task.deadline,
                palette.text,
            ),
            theme::key_value_line(
                palette,
                if en { "Time" } else { "Jam" },
                &task.deadline_time,
                palette.text,
            ),
            theme::key_value_line(
                palette,
                if en { "Status" } else { "Status" },
                task.status_label(app.language_preset()),
                lane_color,
            ),
            theme::key_value_line(
                palette,
                if en { "Countdown" } else { "Hitung Mundur" },
                &match task.hours_until_deadline() {
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
                            "tidak valid".into()
                        }
                    }
                },
                palette.info,
            ),
            theme::key_value_line(
                palette,
                if en { "Delete FX" } else { "Efek Hapus" },
                delete_fx,
                if task.completed {
                    palette.accent
                } else {
                    palette.danger
                },
            ),
            theme::key_value_line(
                palette,
                if en { "Description" } else { "Deskripsi" },
                &task.description,
                palette.text,
            ),
            theme::key_value_line(
                palette,
                if en { "Created" } else { "Dibuat" },
                &task.created_at,
                palette.muted,
            ),
            theme::key_value_line(
                palette,
                if en { "Completed" } else { "Selesai" },
                task.completed_at.as_deref().unwrap_or("-"),
                palette.info,
            ),
            Line::raw(""),
            Line::styled(
                if en {
                    "Actions: a add  |  x done  |  d delete / archive"
                } else {
                    "Aksi: a tambah  |  x selesai  |  d hapus / arsip"
                },
                Style::default().fg(palette.muted),
            ),
            Line::styled(
                if en {
                    "Search: / filter  |  c clear  |  q home"
                } else {
                    "Cari: / filter  |  c clear  |  q home"
                },
                Style::default().fg(palette.muted),
            ),
        ]
    } else {
        vec![
            Line::styled(
                if en {
                    "No task available yet."
                } else {
                    "Belum ada tugas."
                },
                Style::default().fg(palette.muted),
            ),
            Line::styled(
                if en {
                    "Press 'a' to add the first task and light up the board."
                } else {
                    "Tekan 'a' untuk tambah tugas pertama dan hidupkan board."
                },
                Style::default().fg(palette.text),
            ),
        ]
    };

    frame.render_widget(
        Paragraph::new(lines)
            .style(Style::default().fg(palette.text))
            .block(theme::panel_block(
                if en {
                    "Task Inspector"
                } else {
                    "Inspektor Tugas"
                },
                palette,
                palette.info,
                true,
            ))
            .wrap(Wrap { trim: true }),
        area,
    );
}
