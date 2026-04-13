use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

use crate::{app::App, state::LanguagePreset};

use super::{
    forms,
    theme::{self, ThemePalette},
};

pub fn render_language_selector(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let popup = forms::centered_rect(48, 54, area);
    frame.render_widget(Clear, popup);

    let title = if app.language_preset().is_english() {
        "Language Selector"
    } else {
        "Pilih Bahasa"
    };
    let block = theme::modal_block(title, palette);
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(6),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .margin(1)
        .split(inner);

    let active = if app.language_preset().is_english() {
        "Active language: "
    } else {
        "Bahasa aktif: "
    };
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(active, Style::default().fg(palette.muted)),
            Span::styled(
                app.language_preset().label(),
                Style::default()
                    .fg(palette.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        rows[0],
    );

    let entries = LanguagePreset::ALL
        .iter()
        .enumerate()
        .map(|(idx, preset)| language_line(*preset, idx == app.language_selected_index()))
        .collect::<Vec<_>>();
    frame.render_widget(Paragraph::new(entries), rows[1]);

    let preview = if LanguagePreset::ALL[app.language_selected_index()].is_english() {
        "Home screen and settings will use English labels."
    } else {
        "Layar utama dan pengaturan akan memakai label Indonesia."
    };
    frame.render_widget(
        Paragraph::new(preview)
            .alignment(Alignment::Center)
            .block(theme::panel_block("Preview", palette, palette.info, false)),
        rows[2],
    );

    let help = if app.language_preset().is_english() {
        Line::from(vec![
            Span::styled("j/k or arrows", Style::default().fg(palette.text)),
            Span::styled("  |  ", Style::default().fg(palette.muted)),
            Span::styled("Enter apply", Style::default().fg(palette.warn)),
            Span::styled("  |  ", Style::default().fg(palette.muted)),
            Span::styled("Esc cancel", Style::default().fg(palette.muted)),
        ])
    } else {
        Line::from(vec![
            Span::styled("j/k atau panah", Style::default().fg(palette.text)),
            Span::styled("  |  ", Style::default().fg(palette.muted)),
            Span::styled("Enter terapkan", Style::default().fg(palette.warn)),
            Span::styled("  |  ", Style::default().fg(palette.muted)),
            Span::styled("Esc batal", Style::default().fg(palette.muted)),
        ])
    };
    frame.render_widget(Paragraph::new(help).alignment(Alignment::Center), rows[3]);
}

fn language_line(preset: LanguagePreset, selected: bool) -> Line<'static> {
    let marker = if selected { ">" } else { " " };
    let subtitle = match preset {
        LanguagePreset::Indonesian => "default lokal untuk launcher dan pengaturan",
        LanguagePreset::English => "launcher and settings use english labels",
    };

    Line::from(vec![
        Span::styled(
            format!("{marker} "),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{:<10}", preset.label()),
            Style::default().add_modifier(if selected {
                Modifier::BOLD
            } else {
                Modifier::empty()
            }),
        ),
        Span::raw(" "),
        Span::styled(subtitle, Style::default()),
    ])
}
