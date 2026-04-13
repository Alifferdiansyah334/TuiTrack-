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
        if en { "Command Deck" } else { "Dek Perintah" },
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

    let commands = Line::from(vec![
        theme::footer_chip(palette, if en { "1/2/3 mode" } else { "1/2/3 mode" }),
        Span::raw(" "),
        theme::footer_chip(palette, if en { "j/k panel" } else { "j/k panel" }),
        Span::raw(" "),
        theme::footer_chip(palette, if en { "a add flow" } else { "a tambah arus" }),
        Span::raw(" "),
        theme::footer_chip(palette, "b balance"),
        Span::raw(" "),
        theme::footer_chip(palette, "g target"),
        Span::raw(" "),
        theme::footer_chip(palette, "r reset"),
        Span::raw(" "),
        theme::footer_chip(palette, "p page"),
        Span::raw(" "),
        theme::footer_chip(palette, if en { "Enter target" } else { "Enter target" }),
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
    ]);

    let status = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            if en { "Hint: " } else { "Hint: " },
            Style::default().fg(palette.muted),
        ),
        Span::styled(app.mode_hint(), Style::default().fg(palette.text)),
        Span::raw("   "),
        Span::styled(
            if en { "Status: " } else { "Status: " },
            Style::default().fg(palette.muted),
        ),
        Span::styled(app.status().to_string(), Style::default().fg(palette.warn)),
    ])])
    .wrap(Wrap { trim: true });

    frame.render_widget(Paragraph::new(commands).wrap(Wrap { trim: true }), rows[0]);
    frame.render_widget(status, rows[1]);
}
