use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    app::App,
    state::{HomeMenu, ThemePreset},
};

use super::super::theme::{self, ThemePalette};
use super::util::{
    MOTTO_EN, MOTTO_ID, choice_pulse, cycle_word, loading_bar, radar_bar, system_pulse,
};

pub(super) fn render_launchdeck(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    palette: ThemePalette,
    tick: usize,
) {
    let en = app.is_english();
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(68), Constraint::Percentage(32)])
        .split(area);

    let launch_block = theme::panel_block(
        if en { "Launch Deck" } else { "Dek Peluncur" },
        palette,
        palette.info,
        true,
    );
    let launch_inner = launch_block.inner(cols[0]);
    frame.render_widget(launch_block, cols[0]);

    let menu_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints({
            let mut constraints = vec![Constraint::Length(4); HomeMenu::ALL.len()];
            constraints.push(Constraint::Length(2));
            constraints
        })
        .margin(1)
        .split(launch_inner);

    for (idx, choice) in HomeMenu::ALL.into_iter().enumerate() {
        render_menu_card(frame, menu_rows[idx], app, palette, choice, idx, tick);
    }

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::styled("hint", Style::default().fg(palette.muted)),
                Span::raw("  "),
                Span::styled(
                    if app.language_preset().is_english() {
                        "Enter opens a ready module. Binance Tracker now loads live market and account panels."
                    } else {
                        "Enter membuka modul yang siap. Binance Tracker sekarang memuat panel market dan akun."
                    },
                    Style::default().fg(palette.text),
                ),
            ]),
            Line::styled(
                if app.language_preset().is_english() {
                    "Use j/k or arrows to move across launcher cards."
                } else {
                    "Pakai j/k atau panah untuk pindah antar kartu launcher."
                },
                Style::default().fg(palette.muted),
            ),
        ]),
        menu_rows[HomeMenu::ALL.len()],
    );

    render_radar(frame, cols[1], app, palette, tick);
}

pub(super) fn render_boot_strip(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    palette: ThemePalette,
    tick: usize,
) {
    let en = app.is_english();
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(56), Constraint::Percentage(44)])
        .split(area);

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::styled(
                    if en { "boot" } else { "boot" },
                    Style::default().fg(palette.muted),
                ),
                Span::raw("  "),
                Span::styled(loading_bar(tick), Style::default().fg(palette.accent)),
            ]),
            Line::from(vec![
                Span::styled(
                    if en { "scene" } else { "scene" },
                    Style::default().fg(palette.muted),
                ),
                Span::raw("  "),
                Span::styled(system_pulse(tick), Style::default().fg(palette.info)),
            ]),
        ])
        .alignment(Alignment::Center),
        cols[0],
    );

    frame.render_widget(
        Paragraph::new(vec![
            Line::styled(
                if en {
                    "neon finance gateway online"
                } else {
                    "gerbang finansial neon online"
                },
                Style::default().fg(palette.text),
            ),
            Line::styled(
                cycle_word(if en { &MOTTO_EN } else { &MOTTO_ID }, tick),
                Style::default().fg(palette.warn),
            ),
        ])
        .alignment(Alignment::Center),
        cols[1],
    );
}

fn render_menu_card(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    palette: ThemePalette,
    choice: HomeMenu,
    idx: usize,
    tick: usize,
) {
    let active = idx == app.home_selected_index();
    let local = match choice {
        HomeMenu::ExpenseTracker => palette,
        HomeMenu::WorkTracker => theme::palette(ThemePreset::Mono),
        HomeMenu::SecretNotes => theme::palette(ThemePreset::Ocean),
        HomeMenu::BinanceTracker => theme::palette(ThemePreset::Amber),
    };
    let badge = match choice {
        HomeMenu::ExpenseTracker => {
            if app.is_english() {
                "READY"
            } else {
                "SIAP"
            }
        }
        HomeMenu::WorkTracker => {
            if app.is_english() {
                "READY"
            } else {
                "SIAP"
            }
        }
        HomeMenu::SecretNotes => {
            if app.is_english() {
                "LOCKED"
            } else {
                "TERKUNCI"
            }
        }
        HomeMenu::BinanceTracker => {
            if app.is_english() {
                "READY"
            } else {
                "SIAP"
            }
        }
    };
    let highlight = if active { local.accent } else { palette.muted };

    let block = theme::panel_block(
        choice.label(app.language_preset()),
        palette,
        highlight,
        active,
    );
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .margin(1)
        .split(inner);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                if active { ">>" } else { "  " },
                Style::default().fg(local.warn).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                menu_label(choice, app.language_preset().is_english()),
                Style::default().fg(local.text).add_modifier(if active {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
            ),
            Span::raw("   "),
            Span::styled(
                badge,
                Style::default()
                    .fg(if active { Color::Black } else { local.text })
                    .bg(if active {
                        local.warn
                    } else {
                        local.accent_soft
                    }),
            ),
            Span::raw("   "),
            Span::styled(
                choice_pulse(choice, tick, app.is_english()),
                Style::default().fg(local.info),
            ),
        ])),
        rows[0],
    );
    frame.render_widget(
        Paragraph::new(menu_subtitle(choice, app.language_preset().is_english()))
            .style(Style::default().fg(local.muted)),
        rows[1],
    );
}

fn render_radar(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette, tick: usize) {
    let title = if app.language_preset().is_english() {
        "Settings Grid"
    } else {
        "Panel Pengaturan"
    };
    let block = theme::panel_block(title, palette, palette.warn, true);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .margin(1)
        .split(inner);

    let signals = [
        (
            "EXP",
            cycle_word(
                if app.is_english() {
                    &["LIVE", "SYNC", "LOCK"]
                } else {
                    &["AKTIF", "SYNC", "KUNCI"]
                },
                tick,
            ),
        ),
        (
            "SVG",
            cycle_word(
                if app.is_english() {
                    &["FLOW", "STACK", "SAVE"]
                } else {
                    &["ALIR", "TUMPUK", "SIMPAN"]
                },
                tick + 1,
            ),
        ),
        (
            "TGT",
            cycle_word(
                if app.is_english() {
                    &["TRACK", "PUSH", "DONE"]
                } else {
                    &["LACAK", "KEJAR", "SELESAI"]
                },
                tick + 2,
            ),
        ),
        (
            "BAL",
            cycle_word(
                if app.is_english() {
                    &["OPEN", "HOLD", "SAFE"]
                } else {
                    &["BUKA", "TAHAN", "AMAN"]
                },
                tick + 3,
            ),
        ),
    ];

    for (idx, (label, state)) in signals.into_iter().enumerate() {
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    label,
                    Style::default()
                        .fg(palette.text)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
                Span::styled(radar_bar(tick + idx), Style::default().fg(palette.accent)),
                Span::raw(" "),
                Span::styled(state, Style::default().fg(palette.warn)),
            ])),
            rows[idx],
        );
    }

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                if app.is_english() { "THEME" } else { "TEMA" },
                Style::default()
                    .fg(palette.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                app.theme_preset().label(),
                Style::default().fg(palette.info),
            ),
            Span::raw(" "),
            Span::styled("[t]", Style::default().fg(palette.warn)),
        ])),
        rows[4],
    );
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                if app.is_english() { "LANG " } else { "BHS  " },
                Style::default()
                    .fg(palette.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                app.language_preset().label(),
                Style::default().fg(palette.info),
            ),
            Span::raw(" "),
            Span::styled("[l]", Style::default().fg(palette.warn)),
        ])),
        rows[5],
    );
    frame.render_widget(
        Paragraph::new(vec![
            Line::styled(
                if app.language_preset().is_english() {
                    "open settings directly from launcher"
                } else {
                    "buka pengaturan langsung dari launcher"
                },
                Style::default().fg(palette.info),
            ),
            Line::styled(
                if app.language_preset().is_english() {
                    "press t for theme, l for language"
                } else {
                    "tekan t untuk theme, l untuk bahasa"
                },
                Style::default().fg(palette.muted),
            ),
        ])
        .alignment(Alignment::Left),
        rows[7],
    );
}

fn menu_label(choice: HomeMenu, _english: bool) -> &'static str {
    match (choice, _english) {
        (HomeMenu::ExpenseTracker, true) => "Expense Tracker",
        (HomeMenu::ExpenseTracker, false) => "Pelacak Expense",
        (HomeMenu::WorkTracker, true) => "Work Tracker",
        (HomeMenu::WorkTracker, false) => "Pelacak Kerja",
        (HomeMenu::SecretNotes, true) => "Secret Notes",
        (HomeMenu::SecretNotes, false) => "Catatan Rahasia",
        (HomeMenu::BinanceTracker, true) => "Binance Tracker",
        (HomeMenu::BinanceTracker, false) => "Pelacak Binance",
    }
}

fn menu_subtitle(choice: HomeMenu, english: bool) -> &'static str {
    match (choice, english) {
        (HomeMenu::ExpenseTracker, true) => "manage expense, savings, earnings, and balance",
        (HomeMenu::ExpenseTracker, false) => "kelola pengeluaran, tabungan, pemasukan, dan balance",
        (HomeMenu::WorkTracker, true) => "manage tasks, deadlines, and completion",
        (HomeMenu::WorkTracker, false) => "kelola tugas, deadline, dan penyelesaian",
        (HomeMenu::SecretNotes, true) => "store private notes with dedicated passkeys per note",
        (HomeMenu::SecretNotes, false) => "simpan note privat dengan passkey khusus per note",
        (HomeMenu::BinanceTracker, true) => {
            "live Binance watchlist, charts, and spot account snapshot"
        }
        (HomeMenu::BinanceTracker, false) => {
            "watchlist Binance live, chart, dan snapshot akun spot"
        }
    }
}
