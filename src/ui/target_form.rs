use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Clear, Paragraph, Wrap},
};

use crate::{
    app::App,
    state::{TargetForm, TargetFormField, TargetMode},
};

use super::theme::{self, ThemePalette};

pub fn render_target_form(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let form = app.target_form();
    frame.render_widget(Clear, area);

    let block = theme::modal_block(
        if en {
            "Set Budget Target"
        } else {
            "Atur Target Budget"
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
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(5),
        ])
        .split(inner);

    render_title_input(
        frame,
        parts[0],
        &form.title,
        form.focus == TargetFormField::Title,
        en,
        palette,
    );
    render_mode_input(frame, parts[1], form, app.language_preset(), palette);
    render_amount_input(
        frame,
        parts[2],
        &form.amount,
        form.focus == TargetFormField::Amount,
        en,
        palette,
    );

    let help = Paragraph::new(vec![
        Line::styled(
            if en {
                "Fill the target title and choose by saving or by total balance."
            } else {
                "Isi judul target dan pilih by saving atau by total balance."
            },
            Style::default().fg(palette.text),
        ),
        Line::styled(
            if en {
                "Left/right changes option, Tab moves to the next field."
            } else {
                "Kiri/kanan untuk pindah opsi, Tab untuk field berikutnya."
            },
            Style::default().fg(palette.muted),
        ),
        Line::styled(
            if en {
                "Press Enter to save the selected target."
            } else {
                "Enter menyimpan target yang dipilih."
            },
            Style::default().fg(palette.muted),
        ),
    ])
    .block(theme::panel_block(
        if en { "Interaction" } else { "Interaksi" },
        palette,
        palette.warn,
        false,
    ))
    .wrap(Wrap { trim: true });
    frame.render_widget(help, parts[3]);
}

fn render_title_input(
    frame: &mut Frame,
    area: Rect,
    value: &str,
    active: bool,
    en: bool,
    palette: ThemePalette,
) {
    let border = if active { palette.warn } else { palette.muted };
    let widget = Paragraph::new(value.to_string())
        .style(Style::default().fg(palette.text).bg(palette.surface))
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .title(if en { "Target Title" } else { "Judul Target" })
                .border_style(Style::default().fg(border))
                .style(Style::default().bg(palette.surface)),
        );
    frame.render_widget(widget, area);
}

fn render_mode_input(
    frame: &mut Frame,
    area: Rect,
    form: &TargetForm,
    language: crate::state::LanguagePreset,
    palette: ThemePalette,
) {
    let border = if form.focus == TargetFormField::Kind {
        palette.warn
    } else {
        palette.muted
    };

    let line = Line::from(vec![
        mode_chip(
            TargetMode::Saving,
            form.mode == TargetMode::Saving,
            language,
            palette,
        ),
        Span::raw(" "),
        mode_chip(
            TargetMode::TotalBalance,
            form.mode == TargetMode::TotalBalance,
            language,
            palette,
        ),
    ]);

    let widget = Paragraph::new(line)
        .style(Style::default().fg(palette.text).bg(palette.surface))
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .title(if language.is_english() {
                    "Target Type"
                } else {
                    "Jenis Target"
                })
                .border_style(Style::default().fg(border))
                .style(Style::default().bg(palette.surface)),
        );
    frame.render_widget(widget, area);
}

fn render_amount_input(
    frame: &mut Frame,
    area: Rect,
    value: &str,
    active: bool,
    en: bool,
    palette: ThemePalette,
) {
    let border = if active { palette.warn } else { palette.muted };
    let widget = Paragraph::new(value.to_string())
        .style(Style::default().fg(palette.text).bg(palette.surface))
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .title(if en {
                    "Target Amount"
                } else {
                    "Nominal Target"
                })
                .border_style(Style::default().fg(border))
                .style(Style::default().bg(palette.surface)),
        );
    frame.render_widget(widget, area);
}

fn mode_chip(
    mode: TargetMode,
    active: bool,
    language: crate::state::LanguagePreset,
    palette: ThemePalette,
) -> Span<'static> {
    if active {
        Span::styled(
            format!(" {} ", mode.label(language)),
            Style::default()
                .fg(Color::Black)
                .bg(palette.accent)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(
            format!(" {} ", mode.label(language)),
            Style::default().fg(palette.muted).bg(palette.surface),
        )
    }
}
