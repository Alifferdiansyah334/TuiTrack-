use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::state::HomeMenu;

use super::super::theme::ThemePalette;

pub(super) const FULL_LOGO: [&str; 6] = [
    "████████╗██╗   ██╗██╗████████╗██████╗  █████╗  ██████╗██╗  ██╗",
    "╚══██╔══╝██║   ██║██║╚══██╔══╝██╔══██╗██╔══██╗██╔════╝██║ ██╔╝",
    "   ██║   ██║   ██║██║   ██║   ██████╔╝███████║██║     █████╔╝ ",
    "   ██║   ██║   ██║██║   ██║   ██╔══██╗██╔══██║██║     ██╔═██╗ ",
    "   ██║   ╚██████╔╝██║   ██║   ██║  ██║██║  ██║╚██████╗██║  ██╗",
    "   ╚═╝    ╚═════╝ ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝",
];

pub(super) const COMPACT_LOGO: [&str; 4] = [
    "████████╗██╗   ██╗██╗",
    "╚══██╔══╝██║   ██║██║",
    "   ██║   ██║   ██║██║",
    "   ╚═╝    ╚═════╝ ╚═╝",
];

pub(super) const MOTTO_EN: [&str; 4] = [
    "budget with intent",
    "stack your savings",
    "track every move",
    "finish your targets",
];

pub(super) const MOTTO_ID: [&str; 4] = [
    "susun budget dengan niat",
    "tumpuk tabunganmu",
    "catat setiap langkah",
    "selesaikan targetmu",
];

pub(super) fn render_ambient(frame: &mut Frame, area: Rect, palette: ThemePalette, tick: usize) {
    if area.width < 12 || area.height < 3 {
        return;
    }

    let top = Rect::new(area.x, area.y, area.width, 1);
    let bottom = Rect::new(
        area.x,
        area.y + area.height.saturating_sub(1),
        area.width,
        1,
    );
    frame.render_widget(
        Paragraph::new(ambient_line(area.width as usize, tick))
            .style(Style::default().fg(palette.accent_soft)),
        top,
    );
    frame.render_widget(
        Paragraph::new(ambient_line(area.width as usize, tick + 7))
            .style(Style::default().fg(palette.accent_soft)),
        bottom,
    );
}

pub(super) fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

pub(super) fn loading_bar(tick: usize) -> &'static str {
    match tick % 8 {
        0 => "[##------]",
        1 => "[###-----]",
        2 => "[####----]",
        3 => "[#####---]",
        4 => "[######--]",
        5 => "[#######-]",
        6 => "[########]",
        _ => "[#######-]",
    }
}

pub(super) fn system_pulse(tick: usize) -> &'static str {
    match tick % 6 {
        0 => "<idle.....>",
        1 => "<idle..run>",
        2 => "<flow..run>",
        3 => "<flow..max>",
        4 => "<scan..max>",
        _ => "<scan..run>",
    }
}

pub(super) fn rotating_signal(tick: usize, english: bool) -> &'static str {
    match (tick % 4, english) {
        (0, true) => "signal: stable",
        (1, true) => "signal: syncing",
        (2, true) => "signal: online",
        (3, true) => "signal: locked",
        (0, false) => "sinyal: stabil",
        (1, false) => "sinyal: sinkron",
        (2, false) => "sinyal: online",
        _ => "sinyal: terkunci",
    }
}

pub(super) fn tick_badge(tick: usize, english: bool) -> &'static str {
    match (tick % 4, english) {
        (0, true) => "grid: warm",
        (1, true) => "grid: vivid",
        (2, true) => "grid: peak",
        (3, true) => "grid: calm",
        (0, false) => "grid: hangat",
        (1, false) => "grid: hidup",
        (2, false) => "grid: puncak",
        _ => "grid: tenang",
    }
}

pub(super) fn sweep_band(tick: usize) -> String {
    let width = 28;
    let active = tick % width;
    (0..width)
        .map(|idx| {
            if idx == active {
                '='
            } else if idx % 3 == 0 {
                '-'
            } else {
                ' '
            }
        })
        .collect()
}

pub(super) fn choice_pulse(choice: HomeMenu, tick: usize, english: bool) -> &'static str {
    match (choice, english) {
        (HomeMenu::ExpenseTracker, true) => cycle_word(&["FOCUS", "TRACK", "ENTER"], tick),
        (HomeMenu::ExpenseTracker, false) => cycle_word(&["FOKUS", "LACAK", "MASUK"], tick),
        (HomeMenu::WorkTracker, true) => cycle_word(&["TASK", "SHIP", "TRACK"], tick),
        (HomeMenu::WorkTracker, false) => cycle_word(&["TUGAS", "GAS", "LACAK"], tick),
        (HomeMenu::SecretNotes, true) => cycle_word(&["LOCK", "HIDE", "OPEN"], tick),
        (HomeMenu::SecretNotes, false) => cycle_word(&["KUNCI", "SEMBUNYI", "BUKA"], tick),
        (HomeMenu::BinanceTracker, true) => cycle_word(&["COIN", "LIVE", "SOON"], tick),
        (HomeMenu::BinanceTracker, false) => cycle_word(&["KOIN", "LIVE", "NANTI"], tick),
    }
}

pub(super) fn radar_bar(tick: usize) -> &'static str {
    match tick % 4 {
        0 => "▁▃▅▇",
        1 => "▂▄▆█",
        2 => "▃▅▇▇",
        _ => "▄▆█▆",
    }
}

pub(super) fn marquee_line(tick: usize, palette: ThemePalette, english: bool) -> Line<'static> {
    let labels = if english {
        [
            "expense", "saving", "earning", "target", "balance", "theme", "reset",
        ]
    } else {
        [
            "expense",
            "tabungan",
            "pemasukan",
            "target",
            "balance",
            "tema",
            "reset",
        ]
    };
    let active = tick % labels.len();
    let mut spans = Vec::new();

    for (idx, label) in labels.into_iter().enumerate() {
        spans.push(Span::styled(
            format!(" {label} "),
            Style::default().fg(if idx == active {
                palette.warn
            } else if (idx + tick) % 2 == 0 {
                palette.accent
            } else {
                palette.muted
            }),
        ));
        if idx < labels.len() - 1 {
            spans.push(Span::styled("•", Style::default().fg(palette.accent_soft)));
        }
    }

    Line::from(spans)
}

pub(super) fn cycle_word(words: &[&'static str], tick: usize) -> &'static str {
    words[tick % words.len()]
}

fn ambient_line(width: usize, tick: usize) -> String {
    let mut line = String::with_capacity(width);
    for idx in 0..width {
        let ch = match (idx + tick) % 19 {
            0 => '*',
            4 => '.',
            8 => '+',
            13 => '.',
            16 => '·',
            _ => ' ',
        };
        line.push(ch);
    }
    line
}
