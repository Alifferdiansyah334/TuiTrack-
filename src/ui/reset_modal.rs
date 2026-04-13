use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    symbols,
    text::Line,
    widgets::{Clear, Paragraph, Wrap},
};

use crate::app::App;

use super::theme::{self, ThemePalette};

pub fn render_reset_modal(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    frame.render_widget(Clear, area);

    let block = theme::modal_block(
        if en {
            "Reset All Data"
        } else {
            "Reset Semua Data"
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
            Constraint::Min(5),
        ])
        .split(inner);

    frame.render_widget(
        Paragraph::new(app.reset_form().confirmation.clone())
            .style(Style::default().fg(palette.text).bg(palette.surface))
            .block(
                ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_set(symbols::border::ROUNDED)
                    .title(if en {
                        "Type /resetall"
                    } else {
                        "Ketik /resetall"
                    })
                    .border_style(Style::default().fg(palette.danger))
                    .style(Style::default().bg(palette.surface)),
            ),
        parts[0],
    );

    frame.render_widget(
        Paragraph::new(vec![
            Line::styled(
                if en {
                    "This command deletes expense, saving, earning, work tasks, balance, and all targets."
                } else {
                    "Perintah ini akan menghapus expense, saving, pemasukan, work task, balance, dan semua target."
                },
                Style::default().fg(palette.text),
            ),
            Line::styled(
                if en {
                    "To continue, type /resetall exactly and press Enter."
                } else {
                    "Untuk lanjut, ketik persis /resetall lalu tekan Enter."
                },
                Style::default().fg(palette.warn),
            ),
            Line::styled(
                if en {
                    "Press Esc to cancel."
                } else {
                    "Tekan Esc untuk batal."
                },
                Style::default().fg(palette.muted),
            ),
        ])
        .block(theme::panel_block(
            if en { "Confirmation" } else { "Konfirmasi" },
            palette,
            palette.danger,
            false,
        ))
        .wrap(Wrap { trim: true }),
        parts[2],
    );
}
