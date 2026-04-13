use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

use crate::{app::App, state::ThemePreset};

use super::{
    forms,
    theme::{self, ThemePalette},
};

pub fn render_theme_selector(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let popup = forms::centered_rect(52, 62, area);
    frame.render_widget(Clear, popup);

    let block = theme::modal_block(if en { "Theme Selector" } else { "Pemilih Tema" }, palette);
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(8),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .margin(1)
        .split(inner);

    let title = Line::from(vec![
        Span::styled(
            if en {
                "Active theme: "
            } else {
                "Theme aktif: "
            },
            Style::default().fg(palette.muted),
        ),
        Span::styled(
            app.theme_preset().label(),
            Style::default()
                .fg(palette.accent)
                .add_modifier(Modifier::BOLD),
        ),
    ]);
    frame.render_widget(Paragraph::new(title), rows[0]);

    let entries = ThemePreset::ALL
        .iter()
        .enumerate()
        .map(|(idx, preset)| preset_line(*preset, idx == app.theme_selected_index(), en))
        .collect::<Vec<_>>();
    frame.render_widget(
        Paragraph::new(entries)
            .alignment(Alignment::Left)
            .style(Style::default().fg(palette.text)),
        rows[1],
    );

    let preview = theme_preview_line(ThemePreset::ALL[app.theme_selected_index()], en);
    frame.render_widget(
        Paragraph::new(preview)
            .alignment(Alignment::Center)
            .block(theme::panel_block(
                if en { "Preview" } else { "Preview" },
                palette,
                palette.info,
                false,
            )),
        rows[2],
    );

    let help = if en {
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

fn preset_line(preset: ThemePreset, selected: bool, en: bool) -> Line<'static> {
    let swatch = theme::palette(preset);
    let marker = if selected { ">" } else { " " };
    let background = if selected {
        swatch.accent_soft
    } else {
        swatch.surface
    };

    Line::from(vec![
        Span::styled(
            format!("{marker} "),
            Style::default()
                .fg(swatch.accent)
                .bg(background)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("   ", Style::default().bg(swatch.accent)),
        Span::raw(" "),
        Span::styled(
            format!("{:<8}", preset.label()),
            Style::default()
                .fg(swatch.text)
                .bg(background)
                .add_modifier(if selected {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
        ),
        Span::raw(" "),
        Span::styled(
            preset_tagline(preset, en),
            Style::default().fg(swatch.muted).bg(background),
        ),
    ])
}

fn theme_preview_line(preset: ThemePreset, en: bool) -> Line<'static> {
    let swatch = theme::palette(preset);
    Line::from(vec![
        Span::styled("     ", Style::default().bg(swatch.accent)),
        Span::raw(" "),
        Span::styled("     ", Style::default().bg(swatch.warn)),
        Span::raw(" "),
        Span::styled("     ", Style::default().bg(swatch.info)),
        Span::raw(" "),
        Span::styled("     ", Style::default().bg(swatch.danger)),
        Span::raw(" "),
        Span::styled(
            preset_tagline(preset, en),
            Style::default()
                .fg(swatch.text)
                .bg(Color::Reset)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}

fn preset_tagline(preset: ThemePreset, en: bool) -> &'static str {
    match (preset, en) {
        (ThemePreset::Forest, true) => "dark green, data focus",
        (ThemePreset::Forest, false) => "hijau gelap, fokus data",
        (ThemePreset::Amber, true) => "warm, high contrast",
        (ThemePreset::Amber, false) => "hangat, kontras tinggi",
        (ThemePreset::Ocean, true) => "cool blue, modern",
        (ThemePreset::Ocean, false) => "biru dingin, modern",
        (ThemePreset::Mono, true) => "monochrome, minimal",
        (ThemePreset::Mono, false) => "monokrom, minimal",
    }
}
