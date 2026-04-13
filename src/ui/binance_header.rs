use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};

use crate::{
    app::App,
    formatting::{compact_number, format_decimal, format_percent},
    state::{Mode, PanelFocus},
};

use super::theme::{self, ThemePalette};

pub fn render_header(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let is_loading = app.binance_loading();
    let refresh_queued = app.binance_refresh_queued();
    let selected_symbol = app.binance_selected_symbol();
    let selected_interval = app.binance_selected_interval();
    let dashboard = app.binance_dashboard();
    let ticker = app.binance_dashboard().ticker_for(selected_symbol);

    let block = theme::panel_block("TuiTrack Binance", palette, palette.warn, true);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .margin(1)
        .split(inner);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            badge(if en { "SPOT DESK" } else { "DESK SPOT" }, palette.warn),
            Span::raw(" "),
            Span::styled(
                if en {
                    "Binance market board with live pair focus, chart tape, and wallet signal"
                } else {
                    "Board market Binance dengan fokus pair live, tape chart, dan sinyal wallet"
                },
                Style::default()
                    .fg(palette.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("   "),
            badge(format!("PAIR {selected_symbol}").as_str(), palette.info),
            Span::raw(" "),
            badge(format!("TF {selected_interval}").as_str(), palette.accent),
            Span::raw(" "),
            badge(
                if is_loading {
                    if en { "SYNCING" } else { "SYNCING" }
                } else if refresh_queued {
                    if en { "QUEUED" } else { "ANTRI" }
                } else if ticker.is_some_and(|item| item.price_change_percent >= 0.0) {
                    "UPSHIFT"
                } else if ticker.is_some() {
                    "DOWNSHIFT"
                } else {
                    "OFFLINE"
                },
                if is_loading || refresh_queued {
                    palette.info
                } else if ticker.is_some_and(|item| item.price_change_percent >= 0.0) {
                    palette.accent
                } else {
                    palette.danger
                },
            ),
        ])),
        rows[0],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            theme::footer_chip(
                palette,
                if en {
                    "Enter load symbol"
                } else {
                    "Enter muat simbol"
                },
            ),
            Span::raw(" "),
            theme::footer_chip(palette, if en { "u refresh" } else { "u refresh" }),
            Span::raw(" "),
            theme::footer_chip(palette, if en { "< > interval" } else { "< > interval" }),
            Span::raw(" "),
            theme::status_chip(palette, focus_label(app, en), true),
            Span::raw(" "),
            theme::status_chip(
                palette,
                dashboard.account_state.title(en),
                matches!(
                    dashboard.account_state,
                    crate::binance::AccountState::Connected { .. }
                ),
            ),
            Span::raw(" "),
            theme::tab_chip(palette, "t theme", app.mode() == Mode::ThemeSelect),
            Span::raw(" "),
            theme::tab_chip(
                palette,
                if en { "l language" } else { "l bahasa" },
                app.mode() == Mode::LanguageSelect,
            ),
            Span::raw(" "),
            theme::status_chip(
                palette,
                if is_loading {
                    if en {
                        "SYNC IN FLIGHT"
                    } else {
                        "SYNC BERJALAN"
                    }
                } else if refresh_queued {
                    if en { "QUEUE ARMED" } else { "ANTRIAN SIAP" }
                } else if ticker.is_some() {
                    if en { "LIVE FEED" } else { "FEED LIVE" }
                } else if en {
                    "WAITING DATA"
                } else {
                    "MENUNGGU DATA"
                },
                is_loading || refresh_queued || ticker.is_some(),
            ),
        ])),
        rows[1],
    );

    let market_line = match ticker {
        Some(ticker) => Line::from(vec![
            Span::styled(
                if en { "Last " } else { "Harga " },
                Style::default().fg(palette.muted),
            ),
            Span::styled(
                format_decimal(ticker.last_price),
                Style::default().fg(palette.warn),
            ),
            Span::raw("   "),
            Span::styled(
                if en { "24H " } else { "24J " },
                Style::default().fg(palette.muted),
            ),
            Span::styled(
                format_percent(ticker.price_change_percent),
                Style::default().fg(if ticker.price_change_percent >= 0.0 {
                    palette.accent
                } else {
                    palette.danger
                }),
            ),
            Span::raw("   "),
            Span::styled(
                if en { "Band " } else { "Band " },
                Style::default().fg(palette.muted),
            ),
            Span::styled(
                format!(
                    "{} -> {}",
                    format_decimal(ticker.low_price),
                    format_decimal(ticker.high_price)
                ),
                Style::default().fg(palette.text),
            ),
            Span::raw("   "),
            Span::styled(
                if en { "Quote " } else { "Quote " },
                Style::default().fg(palette.muted),
            ),
            Span::styled(
                compact_number(ticker.quote_volume),
                Style::default().fg(palette.info),
            ),
            Span::raw("   "),
            Span::styled(
                if en { "Updated " } else { "Update " },
                Style::default().fg(palette.muted),
            ),
            Span::styled(
                dashboard.last_updated.as_deref().unwrap_or("--"),
                Style::default().fg(if is_loading {
                    palette.info
                } else {
                    palette.accent
                }),
            ),
            Span::raw("   "),
            Span::styled(
                if is_loading {
                    if en {
                        "Sync lane active"
                    } else {
                        "Lane sync aktif"
                    }
                } else if refresh_queued {
                    if en { "Queue ready" } else { "Antrian siap" }
                } else if en {
                    "Feed stable"
                } else {
                    "Feed stabil"
                },
                Style::default().fg(if is_loading || refresh_queued {
                    palette.info
                } else {
                    palette.text
                }),
            ),
        ]),
        None => Line::from(vec![
            Span::styled(
                if en { "Status " } else { "Status " },
                Style::default().fg(palette.muted),
            ),
            Span::styled(app.status().to_string(), Style::default().fg(palette.warn)),
        ]),
    };
    frame.render_widget(
        Paragraph::new(vec![
            market_line,
            Line::from(vec![
                Span::styled(
                    if en { "Desk " } else { "Desk " },
                    Style::default().fg(palette.muted),
                ),
                Span::styled(
                    market_pulse(app, ticker.is_some(), is_loading, refresh_queued),
                    Style::default().fg(if is_loading || refresh_queued {
                        palette.info
                    } else {
                        palette.accent
                    }),
                ),
                Span::raw("   "),
                Span::styled(
                    if en { "Mode " } else { "Mode " },
                    Style::default().fg(palette.muted),
                ),
                Span::styled(app.mode_hint(), Style::default().fg(palette.text)),
            ]),
        ])
        .wrap(Wrap { trim: true }),
        rows[2],
    );
}

pub fn render_summary(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let is_loading = app.binance_loading();
    let refresh_queued = app.binance_refresh_queued();
    let dashboard = app.binance_dashboard();
    let ticker = dashboard.ticker_for(app.binance_selected_symbol());
    let latest_close = dashboard
        .klines
        .last()
        .map(|point| point.close)
        .unwrap_or_default();

    let cards_data = [
        (
            if en { "Pair" } else { "Pair" },
            app.binance_selected_symbol().to_string(),
            if en { "Loaded tape" } else { "Tape aktif" },
            palette.warn,
        ),
        (
            if en { "Spot" } else { "Spot" },
            ticker
                .map(|item| format_decimal(item.last_price))
                .unwrap_or_else(|| "--".into()),
            if en {
                "Current last trade"
            } else {
                "Harga trade terakhir"
            },
            palette.accent,
        ),
        (
            if en { "Momentum" } else { "Momentum" },
            ticker
                .map(|item| format_percent(item.price_change_percent))
                .unwrap_or_else(|| "--".into()),
            if en { "24 hour drift" } else { "Drift 24 jam" },
            if ticker.is_some_and(|item| item.price_change_percent >= 0.0) {
                palette.accent
            } else {
                palette.danger
            },
        ),
        (
            if en { "Close Tape" } else { "Close Tape" },
            if latest_close > 0.0 {
                format_decimal(latest_close)
            } else {
                "--".into()
            },
            if en {
                "Latest loaded candle"
            } else {
                "Candle terakhir"
            },
            palette.info,
        ),
        (
            if en { "Wallet Est." } else { "Estimasi" },
            dashboard
                .wallet_estimate_usdt
                .map(|value| format!("{} USDT", compact_number(value)))
                .unwrap_or_else(|| "--".into()),
            if en { "Spot estimate" } else { "Estimasi spot" },
            palette.warn,
        ),
        (
            if en { "Sync" } else { "Sync" },
            if is_loading {
                if en {
                    "Running".to_string()
                } else {
                    "Berjalan".to_string()
                }
            } else if refresh_queued {
                if en {
                    "Queued".to_string()
                } else {
                    "Antri".to_string()
                }
            } else {
                dashboard.balances.len().to_string()
            },
            if is_loading {
                if en {
                    "Background worker live"
                } else {
                    "Worker background aktif"
                }
            } else if refresh_queued {
                if en {
                    "Next refresh armed"
                } else {
                    "Refresh berikut siap"
                }
            } else if en {
                "Visible holdings"
            } else {
                "Holding terlihat"
            },
            if is_loading || refresh_queued {
                palette.info
            } else {
                Color::Magenta
            },
        ),
    ];

    let cards = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Ratio(1, cards_data.len() as u32);
            cards_data.len()
        ])
        .split(area);

    for (idx, (title, value, subtitle, color)) in cards_data.into_iter().enumerate() {
        theme::render_metric_card(frame, cards[idx], palette, title, &value, subtitle, color);
    }
}

fn badge(label: &str, color: Color) -> Span<'static> {
    Span::styled(
        format!(" {label} "),
        Style::default()
            .fg(Color::Black)
            .bg(color)
            .add_modifier(Modifier::BOLD),
    )
}

fn focus_label(app: &App, english: bool) -> &'static str {
    match app.panel_focus() {
        PanelFocus::Table => {
            if english {
                "Watchlist"
            } else {
                "Watchlist"
            }
        }
        PanelFocus::ExpenseChart => {
            if english {
                "Price Chart"
            } else {
                "Chart Harga"
            }
        }
        PanelFocus::SavingChart => {
            if english {
                "Volume Chart"
            } else {
                "Chart Volume"
            }
        }
        PanelFocus::TargetA => {
            if english {
                "Balances"
            } else {
                "Balance"
            }
        }
        PanelFocus::TargetB => {
            if english {
                "Market Detail"
            } else {
                "Detail Market"
            }
        }
    }
}

fn market_pulse(app: &App, live: bool, loading: bool, queued: bool) -> &'static str {
    let tick = app.animation_tick() % 4;
    match (loading, queued, live, tick) {
        (true, _, _, 0) => "[sync....]",
        (true, _, _, 1) => "[sync==..]",
        (true, _, _, 2) => "[sync====]",
        (true, _, _, _) => "[sync>>>>]",
        (false, true, _, 0) => "[queue...]",
        (false, true, _, 1) => "[queue=..]",
        (false, true, _, 2) => "[queue==.]",
        (false, true, _, _) => "[queue===]",
        (false, false, true, 0) => "[###.....]",
        (false, false, true, 1) => "[#####...]",
        (false, false, true, 2) => "[#######.]",
        (false, false, true, _) => "[########]",
        (false, false, false, 0) => "[..      ]",
        (false, false, false, 1) => "[....    ]",
        (false, false, false, 2) => "[......  ]",
        (false, false, false, _) => "[offline ]",
    }
}
