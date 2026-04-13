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
            "Binance Command Deck"
        } else {
            "Dek Perintah Binance"
        },
        palette,
        palette.warn,
        false,
    );
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(2)])
        .margin(1)
        .split(inner);

    let commands = Line::from(vec![
        theme::footer_chip(palette, if en { "desk nav j/k" } else { "desk nav j/k" }),
        Span::raw(" "),
        theme::footer_chip(palette, if en { "list arrows" } else { "list panah" }),
        Span::raw(" "),
        theme::footer_chip(
            palette,
            if en {
                "Enter arm pair"
            } else {
                "Enter muat pair"
            },
        ),
        Span::raw(" "),
        theme::footer_chip(palette, if en { "< > timeframe" } else { "< > timeframe" }),
        Span::raw(" "),
        theme::footer_chip(palette, if en { "u sync" } else { "u sync" }),
        Span::raw(" "),
        theme::footer_chip(palette, "t theme"),
        Span::raw(" "),
        theme::footer_chip(palette, if en { "l language" } else { "l bahasa" }),
        Span::raw(" "),
        theme::footer_chip(palette, "q home"),
    ]);

    let status = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            if en { "Route: " } else { "Rute: " },
            Style::default().fg(palette.muted),
        ),
        Span::styled(app.mode_hint(), Style::default().fg(palette.text)),
        Span::raw("   "),
        Span::styled(
            if en { "Desk Status: " } else { "Status Desk: " },
            Style::default().fg(palette.muted),
        ),
        Span::styled(app.status().to_string(), Style::default().fg(palette.warn)),
    ])])
    .wrap(Wrap { trim: true });

    frame.render_widget(Paragraph::new(commands).wrap(Wrap { trim: true }), rows[0]);
    frame.render_widget(status, rows[1]);
}
