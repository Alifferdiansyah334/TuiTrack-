use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Cell, Paragraph, Row, Table, TableState, Wrap},
};

use crate::app::App;

use super::theme::{self, ThemePalette};

pub fn render_note_list(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let indices = app.filtered_secret_note_indices();
    let title = if app.filter_is_active() {
        format!(
            "{} ({})  {}: {}",
            if en { "Vault Index" } else { "Indeks Vault" },
            indices.len(),
            if en { "Filter" } else { "Filter" },
            app.filter()
        )
    } else {
        format!(
            "{} ({})",
            if en { "Vault Index" } else { "Indeks Vault" },
            indices.len()
        )
    };

    let rows = indices
        .iter()
        .enumerate()
        .filter_map(|(row_idx, idx)| app.secret_note_at(*idx).map(|note| (row_idx, note)))
        .map(|(row_idx, note)| {
            let unlocked = app.unlocked_secret_content(note.id).is_some();
            let row_bg = if row_idx % 2 == 0 {
                palette.surface
            } else {
                palette.accent_soft
            };
            Row::new(vec![
                Cell::from(Line::from(vec![lock_badge(unlocked)])),
                Cell::from(note.created_at.clone()),
                Cell::from(Line::from(vec![Span::styled(
                    note.title.clone(),
                    Style::default()
                        .fg(if unlocked {
                            Color::Rgb(117, 239, 255)
                        } else {
                            palette.text
                        })
                        .add_modifier(if unlocked {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        }),
                )])),
            ])
            .style(Style::default().bg(row_bg))
        });

    let table = Table::new(
        rows,
        [
            ratatui::layout::Constraint::Length(10),
            ratatui::layout::Constraint::Length(18),
            ratatui::layout::Constraint::Min(16),
        ],
    )
    .header(
        Row::new(vec![
            if en { "Lock" } else { "Kunci" },
            if en { "Created" } else { "Dibuat" },
            if en { "Title" } else { "Judul" },
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
        Color::Rgb(117, 239, 255),
        true,
    ))
    .row_highlight_style(
        Style::default()
            .bg(Color::Rgb(28, 66, 78))
            .fg(palette.text)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ");

    let mut state = TableState::default();
    if !indices.is_empty() {
        state.select(Some(app.secret_selected_index()));
    }
    frame.render_stateful_widget(table, area, &mut state);
}

pub fn render_vault_status(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let lines = vec![
        theme::key_value_line(
            palette,
            if en { "Stored" } else { "Tersimpan" },
            &app.secret_note_count().to_string(),
            palette.text,
        ),
        theme::key_value_line(
            palette,
            if en { "Locked" } else { "Terkunci" },
            &app.locked_secret_note_count().to_string(),
            Color::Rgb(255, 110, 161),
        ),
        theme::key_value_line(
            palette,
            if en { "Unlocked" } else { "Terbuka" },
            &app.unlocked_secret_note_count().to_string(),
            Color::Rgb(117, 239, 255),
        ),
        theme::key_value_line(
            palette,
            if en { "Visible chars" } else { "Karakter buka" },
            &app.secret_note_payload_chars().to_string(),
            palette.warn,
        ),
    ];

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme::panel_block(
                if en { "Vault Status" } else { "Status Vault" },
                palette,
                Color::Rgb(255, 110, 161),
                false,
            ))
            .wrap(Wrap { trim: true }),
        area,
    );
}

pub fn render_note_preview(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let Some(note) = app.selected_secret_note() else {
        frame.render_widget(
            Paragraph::new(vec![
                Line::styled(
                    if en {
                        "No secret note selected."
                    } else {
                        "Belum ada catatan rahasia terpilih."
                    },
                    Style::default().fg(palette.text),
                ),
                Line::styled(
                    if en {
                        "Press 'a' to create the first vault entry."
                    } else {
                        "Tekan 'a' untuk membuat note vault pertama."
                    },
                    Style::default().fg(palette.muted),
                ),
            ])
            .block(theme::panel_block(
                if en {
                    "Cipher Preview"
                } else {
                    "Preview Cipher"
                },
                palette,
                Color::Rgb(117, 239, 255),
                true,
            ))
            .wrap(Wrap { trim: true }),
            area,
        );
        return;
    };

    let unlocked = app.unlocked_secret_content(note.id);
    let mut lines = vec![
        Line::from(vec![
            Span::styled(
                if en { " TITLE " } else { " JUDUL " },
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Rgb(117, 239, 255))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                note.title.clone(),
                Style::default()
                    .fg(palette.text)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::raw(""),
    ];

    if let Some(content) = unlocked {
        lines.push(Line::styled(
            if en {
                "DECRYPTED CONTENT"
            } else {
                "KONTEN TERDEKRIPSI"
            },
            Style::default()
                .fg(Color::Rgb(117, 239, 255))
                .add_modifier(Modifier::BOLD),
        ));
        lines.push(Line::raw(""));
        lines.push(Line::styled(
            content.to_string(),
            Style::default().fg(palette.text),
        ));
    } else {
        lines.push(Line::styled(
            if en {
                "CONTENT LOCKED"
            } else {
                "KONTEN TERKUNCI"
            },
            Style::default()
                .fg(Color::Rgb(255, 110, 161))
                .add_modifier(Modifier::BOLD),
        ));
        lines.push(Line::raw(""));
        lines.push(Line::styled(
            if en {
                "Use 'u' or Enter and provide the passkey assigned when this note was created."
            } else {
                "Gunakan 'u' atau Enter lalu masukkan passkey yang diset saat note ini dibuat."
            },
            Style::default().fg(palette.text),
        ));
        lines.push(Line::raw(""));
        lines.push(Line::styled(
            "████████████████████████████████████████████",
            Style::default().fg(palette.muted),
        ));
    }

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme::panel_block(
                if en {
                    "Cipher Preview"
                } else {
                    "Preview Cipher"
                },
                palette,
                Color::Rgb(117, 239, 255),
                true,
            ))
            .wrap(Wrap { trim: true }),
        area,
    );
}

pub fn render_note_activity(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let lines = if let Some(note) = app.selected_secret_note() {
        vec![
            theme::key_value_line(
                palette,
                if en { "State" } else { "Status" },
                if app.unlocked_secret_content(note.id).is_some() {
                    if en { "Unlocked" } else { "Terbuka" }
                } else {
                    if en { "Locked" } else { "Terkunci" }
                },
                if app.unlocked_secret_content(note.id).is_some() {
                    Color::Rgb(117, 239, 255)
                } else {
                    Color::Rgb(255, 110, 161)
                },
            ),
            theme::key_value_line(
                palette,
                if en { "Created" } else { "Dibuat" },
                &note.created_at,
                palette.text,
            ),
            theme::key_value_line(
                palette,
                if en { "Last unlock" } else { "Buka terakhir" },
                note.last_unlocked_at.as_deref().unwrap_or("-"),
                palette.warn,
            ),
            theme::key_value_line(
                palette,
                if en { "Security" } else { "Keamanan" },
                if en {
                    "Dedicated passkey per note"
                } else {
                    "Passkey khusus tiap note"
                },
                palette.info,
            ),
        ]
    } else {
        vec![Line::styled(
            if en {
                "Select a note to inspect its vault metadata."
            } else {
                "Pilih note untuk melihat metadata vault-nya."
            },
            Style::default().fg(palette.text),
        )]
    };

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme::panel_block(
                if en { "Access Log" } else { "Log Akses" },
                palette,
                Color::Rgb(255, 110, 161),
                false,
            ))
            .wrap(Wrap { trim: true }),
        area,
    );
}

fn lock_badge(unlocked: bool) -> Span<'static> {
    let (label, color) = if unlocked {
        (" OPEN ", Color::Rgb(117, 239, 255))
    } else {
        (" LOCK ", Color::Rgb(255, 110, 161))
    };
    Span::styled(
        label,
        Style::default()
            .fg(Color::Black)
            .bg(color)
            .add_modifier(Modifier::BOLD),
    )
}
