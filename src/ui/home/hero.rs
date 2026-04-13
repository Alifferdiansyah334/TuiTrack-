use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};

use super::super::theme::ThemePalette;
use super::util::{COMPACT_LOGO, FULL_LOGO, marquee_line, rotating_signal, sweep_band, tick_badge};

pub(super) fn render_statusline(
    frame: &mut Frame,
    area: Rect,
    palette: ThemePalette,
    tick: usize,
    english: bool,
) {
    let subtitle = if english {
        "personal finance cockpit"
    } else {
        "kokpit finansial personal"
    };
    let line = Line::from(vec![
        Span::styled(
            " LIVE ",
            Style::default().fg(Color::Black).bg(palette.accent),
        ),
        Span::raw(" "),
        Span::styled(subtitle, Style::default().fg(palette.text)),
        Span::raw("   "),
        Span::styled(
            rotating_signal(tick, english),
            Style::default().fg(palette.warn),
        ),
        Span::raw("   "),
        Span::styled(tick_badge(tick, english), Style::default().fg(palette.info)),
    ]);
    frame.render_widget(Paragraph::new(line).alignment(Alignment::Center), area);
}

pub(super) fn render_banner(frame: &mut Frame, area: Rect, palette: ThemePalette, tick: usize) {
    let line = Line::from(vec![
        Span::styled("◈", Style::default().fg(palette.warn)),
        Span::raw(" "),
        Span::styled(sweep_band(tick), Style::default().fg(palette.accent_soft)),
        Span::raw(" "),
        Span::styled("◈", Style::default().fg(palette.warn)),
    ]);
    frame.render_widget(Paragraph::new(line).alignment(Alignment::Center), area);
}

pub(super) fn render_logo(
    frame: &mut Frame,
    area: Rect,
    width: u16,
    palette: ThemePalette,
    tick: usize,
) {
    let hero = if width >= 74 {
        FULL_LOGO.as_slice()
    } else {
        COMPACT_LOGO.as_slice()
    };
    let visible = (tick / 2 + 1).min(hero.len());
    let glow_row = tick % hero.len();

    let lines = hero
        .iter()
        .take(visible)
        .enumerate()
        .map(|(idx, line)| {
            let color = if idx == glow_row {
                palette.warn
            } else if idx % 2 == 0 {
                palette.accent
            } else {
                palette.info
            };
            Line::styled(
                *line,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            )
        })
        .collect::<Vec<_>>();

    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), area);
}

pub(super) fn render_subtitle(frame: &mut Frame, area: Rect, palette: ThemePalette, english: bool) {
    let text = if english {
        "expense tracker • savings • earnings • target radar • balance board"
    } else {
        "pelacak expense • tabungan • pemasukan • radar target • papan balance"
    };
    frame.render_widget(
        Paragraph::new(text).alignment(Alignment::Center).style(
            Style::default()
                .fg(palette.text)
                .add_modifier(Modifier::BOLD),
        ),
        area,
    );
}

pub(super) fn render_feature_ribbon(
    frame: &mut Frame,
    area: Rect,
    palette: ThemePalette,
    tick: usize,
    english: bool,
) {
    frame.render_widget(
        Paragraph::new(marquee_line(tick, palette, english))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        area,
    );
}

pub(super) fn render_help(frame: &mut Frame, area: Rect, palette: ThemePalette, english: bool) {
    let line_a = if english {
        "j/k or arrows move menu, t theme, l language"
    } else {
        "j/k atau panah pindah menu, t theme, l bahasa"
    };
    let line_b = if english {
        "Enter opens Expense Tracker, q exits launcher"
    } else {
        "Enter buka Expense Tracker, q keluar dari launcher"
    };
    frame.render_widget(
        Paragraph::new(vec![
            Line::styled(line_a, Style::default().fg(palette.text)),
            Line::styled(line_b, Style::default().fg(palette.muted)),
        ])
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true }),
        area,
    );
}
