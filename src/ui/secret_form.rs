use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Clear, Paragraph, Wrap},
};

use crate::{app::App, state::SecretNoteFormField};

use super::theme::{self, ThemePalette};

pub fn render_secret_note_form(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let form = app.secret_note_form();
    let editing = app.mode() == crate::state::Mode::EditSecretNote;
    frame.render_widget(Clear, area);

    let block = theme::modal_block(
        if editing {
            if en {
                "Edit Secret Note"
            } else {
                "Edit Catatan Rahasia"
            }
        } else {
            if en {
                "Add Secret Note"
            } else {
                "Tambah Catatan Rahasia"
            }
        },
        palette,
    );
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let parts = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Length(3),
            Constraint::Min(4),
        ])
        .split(inner);

    render_input(
        frame,
        parts[0],
        if en { "Vault Title" } else { "Judul Vault" },
        &form.title,
        form.focus == SecretNoteFormField::Title,
        false,
        palette,
    );
    render_input(
        frame,
        parts[1],
        if en { "Secret Content" } else { "Isi Rahasia" },
        &form.content,
        form.focus == SecretNoteFormField::Content,
        false,
        palette,
    );
    render_input(
        frame,
        parts[2],
        if en { "Passkey" } else { "Passkey" },
        &mask_value(&form.passkey),
        form.focus == SecretNoteFormField::Passkey,
        true,
        palette,
    );

    let help = Paragraph::new(vec![
        Line::styled(
            if editing {
                if en {
                    "The note must already be unlocked. Enter the same note passkey again to re-encrypt the updated content."
                } else {
                    "Note harus sudah terbuka. Masukkan lagi passkey note yang sama untuk mengenkripsi ulang isi yang diperbarui."
                }
            } else {
                if en {
                    "Each note gets its own passkey. The stored content is encrypted before it is written to disk."
                } else {
                    "Setiap note memakai passkey sendiri. Konten disimpan dalam bentuk terenkripsi sebelum ditulis ke disk."
                }
            },
            Style::default().fg(palette.text),
        ),
        Line::styled(
            if editing {
                if en {
                    "Tab moves fields. Press Enter on passkey to save edits. Esc cancels."
                } else {
                    "Tab pindah field. Tekan Enter di passkey untuk simpan edit. Esc membatalkan."
                }
            } else {
                if en {
                    "Tab moves fields. Press Enter on passkey to store the note. Esc cancels."
                } else {
                    "Tab pindah field. Tekan Enter di passkey untuk simpan note. Esc membatalkan."
                }
            },
            Style::default().fg(palette.muted),
        ),
    ])
    .block(theme::panel_block(
        if en { "Vault Rules" } else { "Aturan Vault" },
        palette,
        Color::Rgb(117, 239, 255),
        false,
    ))
    .wrap(Wrap { trim: true });
    frame.render_widget(help, parts[3]);
}

pub fn render_unlock_note_form(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    frame.render_widget(Clear, area);

    let block = theme::modal_block(
        if en {
            "Unlock Secret Note"
        } else {
            "Buka Catatan Rahasia"
        },
        palette,
    );
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let parts = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Min(3),
        ])
        .split(inner);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                if en { "Target: " } else { "Target: " },
                Style::default().fg(palette.muted),
            ),
            Span::styled(
                app.pending_unlock_note_title().unwrap_or("-"),
                Style::default()
                    .fg(palette.text)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        parts[0],
    );
    render_input(
        frame,
        parts[1],
        if en { "Passkey" } else { "Passkey" },
        &mask_value(&app.unlock_note_form().passkey),
        true,
        true,
        palette,
    );
    frame.render_widget(
        Paragraph::new(vec![
            Line::styled(
                if en {
                    "Enter the note-specific passkey to decrypt and reveal the content."
                } else {
                    "Masukkan passkey khusus note untuk mendekripsi dan menampilkan isinya."
                },
                Style::default().fg(palette.text),
            ),
            Line::styled(
                if en {
                    "Enter unlocks the note. Esc cancels."
                } else {
                    "Enter membuka note. Esc membatalkan."
                },
                Style::default().fg(palette.muted),
            ),
        ])
        .wrap(Wrap { trim: true }),
        parts[2],
    );
}

fn render_input(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    value: &str,
    active: bool,
    secret: bool,
    palette: ThemePalette,
) {
    let border = if active {
        Color::Rgb(117, 239, 255)
    } else {
        palette.muted
    };
    let title_style = if secret {
        Style::default().fg(Color::Rgb(255, 110, 161))
    } else {
        Style::default().fg(palette.text)
    };
    frame.render_widget(
        Paragraph::new(value.to_string())
            .style(Style::default().fg(palette.text).bg(palette.surface))
            .wrap(Wrap { trim: true })
            .block(
                ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_set(symbols::border::ROUNDED)
                    .title(Span::styled(format!(" {title} "), title_style))
                    .border_style(Style::default().fg(border))
                    .style(Style::default().bg(palette.surface)),
            ),
        area,
    );
}

fn mask_value(value: &str) -> String {
    if value.is_empty() {
        String::new()
    } else {
        "•".repeat(value.chars().count())
    }
}
