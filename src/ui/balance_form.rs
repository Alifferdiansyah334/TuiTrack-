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
    state::{BalanceForm, BalanceFormField},
};

use super::theme::{self, ThemePalette};

pub fn render_balance_form(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let form = app.balance_form();
    frame.render_widget(Clear, area);

    let block = theme::modal_block(if en { "Set Balance" } else { "Atur Balance" }, palette);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let parts = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(5),
        ])
        .split(inner);

    render_toggle_input(frame, parts[0], form, en, palette);
    render_amount_input(
        frame,
        parts[1],
        &form.amount,
        form.focus == BalanceFormField::Amount,
        en,
        palette,
    );

    let help = Paragraph::new(vec![
        Line::styled(
            if en {
                "Toggle balance with left/right or space."
            } else {
                "Toggle balance dengan kiri/kanan atau spasi."
            },
            Style::default().fg(palette.text),
        ),
        Line::styled(
            if en {
                "Enter an amount to set the base balance. Leave blank to only toggle."
            } else {
                "Isi nominal untuk set balance dasar. Kosongkan jika hanya toggle."
            },
            Style::default().fg(palette.muted),
        ),
        Line::styled(
            if en {
                "Press Enter to save. If disabled, result becomes savings + earnings - expense."
            } else {
                "Enter simpan. Jika balance off, hasil jadi tabungan + pemasukan - expense."
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
    frame.render_widget(help, parts[2]);
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
                .title(if en { "Set Amount" } else { "Set Nominal" })
                .border_style(Style::default().fg(border))
                .style(Style::default().bg(palette.surface)),
        );
    frame.render_widget(widget, area);
}

fn render_toggle_input(
    frame: &mut Frame,
    area: Rect,
    form: &BalanceForm,
    en: bool,
    palette: ThemePalette,
) {
    let border = if form.focus == BalanceFormField::Enabled {
        palette.warn
    } else {
        palette.muted
    };

    let line = Line::from(vec![
        toggle_chip(if en { "ON" } else { "ON" }, form.enabled, palette),
        Span::raw(" "),
        toggle_chip(if en { "OFF" } else { "OFF" }, !form.enabled, palette),
    ]);

    let widget = Paragraph::new(line)
        .style(Style::default().fg(palette.text).bg(palette.surface))
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .title(if en {
                    "Use Base Balance"
                } else {
                    "Gunakan Balance Dasar"
                })
                .border_style(Style::default().fg(border))
                .style(Style::default().bg(palette.surface)),
        );
    frame.render_widget(widget, area);
}

fn toggle_chip(label: &str, active: bool, palette: ThemePalette) -> Span<'static> {
    if active {
        Span::styled(
            format!(" {label} "),
            Style::default()
                .fg(Color::Black)
                .bg(palette.accent)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(
            format!(" {label} "),
            Style::default().fg(palette.muted).bg(palette.surface),
        )
    }
}
