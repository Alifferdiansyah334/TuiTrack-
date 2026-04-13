use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};

use crate::app::App;

use super::theme::{self, ThemePalette};

pub fn render_footer(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let block = theme::panel_block(
        if en {
            "Vault Commands"
        } else {
            "Perintah Vault"
        },
        palette,
        palette.info,
        false,
    );
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(2)])
        .margin(1)
        .split(inner);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            theme::footer_chip(palette, if en { "arrows select" } else { "panah pilih" }),
            Span::raw(" "),
            theme::footer_chip(palette, if en { "a add note" } else { "a tambah note" }),
            Span::raw(" "),
            theme::footer_chip(palette, if en { "e edit note" } else { "e edit note" }),
            Span::raw(" "),
            theme::footer_chip(
                palette,
                if en {
                    "u or Enter unlock/lock"
                } else {
                    "u atau Enter buka/kunci"
                },
            ),
            Span::raw(" "),
            theme::footer_chip(palette, if en { "d delete" } else { "d hapus" }),
            Span::raw(" "),
            theme::footer_chip(palette, if en { "/ filter" } else { "/ filter" }),
            Span::raw(" "),
            theme::footer_chip(palette, if en { "c clear" } else { "c clear" }),
            Span::raw(" "),
            theme::footer_chip(palette, "t theme"),
            Span::raw(" "),
            theme::footer_chip(palette, if en { "l language" } else { "l bahasa" }),
            Span::raw(" "),
            theme::footer_chip(palette, "q home"),
        ]))
        .wrap(Wrap { trim: true }),
        rows[0],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("Hint: ", Style::default().fg(palette.muted)),
            Span::styled(app.mode_hint(), Style::default().fg(palette.text)),
            Span::raw("   "),
            Span::styled("Status: ", Style::default().fg(palette.muted)),
            Span::styled(app.status().to_string(), Style::default().fg(palette.warn)),
        ]))
        .wrap(Wrap { trim: true }),
        rows[1],
    );
}
