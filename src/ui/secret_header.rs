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
    let block = theme::panel_block("TuiTrack Vault", palette, Color::Rgb(117, 239, 255), true);
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
                if en {
                    "Secret Notes Vault"
                } else {
                    "Vault Catatan Rahasia"
                },
                Style::default()
                    .fg(palette.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                if en {
                    "per-note passkey vault with locked previews and private unlock flow"
                } else {
                    "vault passkey per note dengan preview terkunci dan alur unlock privat"
                },
                Style::default().fg(palette.muted),
            ),
            Span::raw("   "),
            metric_pill(
                if en {
                    format!("LOCK {}", app.locked_secret_note_count())
                } else {
                    format!("KUNCI {}", app.locked_secret_note_count())
                },
                Color::Rgb(255, 110, 161),
            ),
            Span::raw(" "),
            metric_pill(
                if en {
                    format!("OPEN {}", app.unlocked_secret_note_count())
                } else {
                    format!("BUKA {}", app.unlocked_secret_note_count())
                },
                Color::Rgb(117, 239, 255),
            ),
        ])),
        rows[0],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            theme::tab_chip(
                palette,
                if en { "a Add Vault" } else { "a Tambah Vault" },
                app.mode() == crate::state::Mode::AddSecretNote,
            ),
            Span::raw(" "),
            theme::tab_chip(
                palette,
                if en { "e Edit" } else { "e Edit" },
                app.mode() == crate::state::Mode::EditSecretNote,
            ),
            Span::raw(" "),
            theme::tab_chip(
                palette,
                if en { "u Unlock/Lock" } else { "u Buka/Kunci" },
                app.mode() == crate::state::Mode::UnlockSecretNote
                    || app.selected_secret_note_is_unlocked(),
            ),
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
        ])),
        rows[1],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("Hint: ", Style::default().fg(palette.muted)),
            Span::styled(app.mode_hint(), Style::default().fg(palette.text)),
            Span::raw("   "),
            Span::styled("Status: ", Style::default().fg(palette.muted)),
            Span::styled(
                app.status().to_string(),
                Style::default().fg(Color::Rgb(117, 239, 255)),
            ),
        ])),
        rows[2],
    );
}

pub fn render_summary(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let selected_state = match app.selected_secret_note() {
        Some(note) if app.unlocked_secret_content(note.id).is_some() => {
            if en {
                "Unlocked"
            } else {
                "Terbuka"
            }
        }
        Some(_) => {
            if en {
                "Locked"
            } else {
                "Terkunci"
            }
        }
        None => "-",
    };
    let last_touch = app
        .selected_secret_note()
        .and_then(|note| note.last_unlocked_at.as_deref())
        .unwrap_or("-");

    let cards = [
        (
            if en { "Notes" } else { "Note" },
            app.secret_note_count().to_string(),
            if en {
                "Stored in vault"
            } else {
                "Tersimpan di vault"
            },
            Color::Rgb(117, 239, 255),
        ),
        (
            if en { "Locked" } else { "Terkunci" },
            app.locked_secret_note_count().to_string(),
            if en {
                "Still encrypted"
            } else {
                "Masih terenkripsi"
            },
            Color::Rgb(255, 110, 161),
        ),
        (
            if en { "Unlocked" } else { "Terbuka" },
            app.unlocked_secret_note_count().to_string(),
            if en {
                "Live this session"
            } else {
                "Aktif sesi ini"
            },
            Color::Rgb(117, 239, 255),
        ),
        (
            if en { "Selected" } else { "Terpilih" },
            selected_state.to_string(),
            if en {
                "Current note state"
            } else {
                "Status note saat ini"
            },
            palette.warn,
        ),
        (
            if en { "Payload" } else { "Payload" },
            format!("{} ch", app.secret_note_payload_chars()),
            if en {
                "Visible decrypted chars"
            } else {
                "Karakter terbuka"
            },
            palette.info,
        ),
        (
            if en { "Last Unlock" } else { "Buka Terakhir" },
            last_touch.to_string(),
            if en {
                "For selected note"
            } else {
                "Untuk note terpilih"
            },
            palette.accent,
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

fn metric_pill(label: String, color: Color) -> Span<'static> {
    Span::styled(
        format!(" {label} "),
        Style::default()
            .fg(Color::Black)
            .bg(color)
            .add_modifier(Modifier::BOLD),
    )
}
