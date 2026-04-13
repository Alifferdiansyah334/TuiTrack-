use chrono::{Local, TimeZone};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Bar, BarChart, BarGroup, Chart, Dataset, GraphType, Paragraph, Wrap},
};

use crate::{
    app::App,
    formatting::{compact_number, format_decimal, format_percent},
    state::PanelFocus,
};

use super::theme::{self, ThemePalette};

pub fn render_body(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(area);
    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(66), Constraint::Percentage(34)])
        .split(rows[0]);
    let price_cols = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(top[0]);
    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(38),
            Constraint::Percentage(30),
            Constraint::Percentage(32),
        ])
        .split(rows[1]);

    render_price_chart(frame, price_cols[0], app, palette);
    render_volume_chart(frame, price_cols[1], app, palette);
    render_market_detail(frame, top[1], app, palette);
    render_watchlist(frame, bottom[0], app, palette);
    render_balances(frame, bottom[1], app, palette);
    render_account_panel(frame, bottom[2], app, palette);
}

fn render_price_chart(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let is_loading = app.binance_loading();
    let block = theme::panel_block(
        if app.is_english() {
            "Price Chart"
        } else {
            "Chart Harga"
        },
        palette,
        palette.warn,
        app.panel_focus() == PanelFocus::ExpenseChart,
    );
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 8 || inner.height < 6 {
        return;
    }

    if app.binance_dashboard().klines.len() < 2 {
        render_empty(
            frame,
            inner,
            app,
            if is_loading && app.is_english() {
                "Syncing candle lane in the background."
            } else if is_loading {
                "Sedang sync jalur candle di background."
            } else if app.is_english() {
                "No price candles loaded yet."
            } else {
                "Belum ada candle harga yang dimuat."
            },
            palette,
        );
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(4)])
        .margin(1)
        .split(inner);

    let points = app
        .binance_dashboard()
        .klines
        .iter()
        .enumerate()
        .map(|(idx, point)| (idx as f64, point.close))
        .collect::<Vec<_>>();
    let min = points
        .iter()
        .map(|(_, price)| *price)
        .fold(f64::INFINITY, f64::min);
    let max = points
        .iter()
        .map(|(_, price)| *price)
        .fold(f64::NEG_INFINITY, f64::max);
    let spread = (max - min).abs().max((max.abs() * 0.005).max(0.0001));

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                badge(
                    if app.is_english() {
                        "PRICE TAPE"
                    } else {
                        "TAPE HARGA"
                    },
                    palette.warn,
                ),
                Span::raw(" "),
                Span::styled(
                    format!(
                        "{}  {}",
                        app.binance_selected_symbol(),
                        app.binance_selected_interval()
                    ),
                    Style::default()
                        .fg(palette.text)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("   "),
                Span::styled(
                    trend_tape(
                        &app.binance_dashboard()
                            .klines
                            .iter()
                            .map(|point| point.close)
                            .collect::<Vec<_>>(),
                    ),
                    Style::default().fg(palette.info),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    if app.is_english() {
                        "Close line across the loaded candle window"
                    } else {
                        "Garis close untuk jendela candle yang dimuat"
                    },
                    Style::default().fg(palette.muted),
                ),
                Span::raw("   "),
                Span::styled(
                    format!(
                        "{} {}",
                        if app.is_english() { "Range" } else { "Range" },
                        format!("{} -> {}", format_decimal(min), format_decimal(max))
                    ),
                    Style::default().fg(palette.text),
                ),
            ]),
        ])
        .wrap(Wrap { trim: true }),
        rows[0],
    );

    frame.render_widget(
        Paragraph::new("").style(Style::default().bg(palette.surface)),
        rows[1],
    );

    let datasets = vec![
        Dataset::default()
            .name("close")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(palette.warn).bg(palette.surface))
            .graph_type(GraphType::Line)
            .data(&points),
    ];
    let chart = Chart::new(datasets)
        .style(Style::default().fg(palette.text).bg(palette.surface))
        .x_axis(
            Axis::default()
                .title(if app.is_english() {
                    "candles"
                } else {
                    "candle"
                })
                .style(Style::default().fg(palette.muted).bg(palette.surface))
                .bounds([0.0, (points.len().saturating_sub(1)) as f64]),
        )
        .y_axis(
            Axis::default()
                .title(if app.is_english() { "price" } else { "harga" })
                .style(Style::default().fg(palette.muted).bg(palette.surface))
                .bounds([min - spread * 0.15, max + spread * 0.15]),
        );
    frame.render_widget(chart, rows[1]);
}

fn render_volume_chart(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let is_loading = app.binance_loading();
    let block = theme::panel_block(
        if app.is_english() {
            "Volume Chart"
        } else {
            "Chart Volume"
        },
        palette,
        palette.info,
        app.panel_focus() == PanelFocus::SavingChart,
    );
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 8 || inner.height < 6 {
        return;
    }

    let latest = app
        .binance_dashboard()
        .klines
        .iter()
        .rev()
        .take(12)
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
    if latest.is_empty() {
        render_empty(
            frame,
            inner,
            app,
            if is_loading && app.is_english() {
                "Syncing volume bars in the background."
            } else if is_loading {
                "Sedang sync bar volume di background."
            } else if app.is_english() {
                "No volume data yet."
            } else {
                "Belum ada data volume."
            },
            palette,
        );
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(4)])
        .margin(1)
        .split(inner);

    let max_volume = latest.iter().map(|point| point.volume).fold(0.0, f64::max);
    let bars = latest
        .iter()
        .enumerate()
        .map(|(idx, point)| {
            Bar::default()
                .label(Line::from(short_time(
                    point.open_time,
                    app.binance_selected_interval(),
                )))
                .value(app.animated_chart_value(point.volume, idx))
                .text_value(compact_number(point.volume))
        })
        .collect::<Vec<_>>();

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                badge(
                    if app.is_english() { "VOLUME" } else { "VOLUME" },
                    palette.info,
                ),
                Span::raw(" "),
                Span::styled(
                    if app.is_english() {
                        "latest 12 candles"
                    } else {
                        "12 candle terakhir"
                    },
                    Style::default().fg(palette.muted),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    if app.is_english() { "Peak " } else { "Puncak " },
                    Style::default().fg(palette.muted),
                ),
                Span::styled(
                    compact_number(max_volume),
                    Style::default().fg(palette.text),
                ),
                Span::raw("   "),
                Span::styled(
                    if app.is_english() { "Avg " } else { "Rata " },
                    Style::default().fg(palette.muted),
                ),
                Span::styled(
                    compact_number(
                        latest.iter().map(|point| point.volume).sum::<f64>() / latest.len() as f64,
                    ),
                    Style::default().fg(palette.warn),
                ),
            ]),
        ])
        .wrap(Wrap { trim: true }),
        rows[0],
    );

    frame.render_widget(
        Paragraph::new("").style(Style::default().bg(palette.surface)),
        rows[1],
    );

    let chart = BarChart::default()
        .direction(Direction::Vertical)
        .bar_width(3)
        .bar_gap(1)
        .group_gap(0)
        .bar_style(Style::default().fg(palette.info).bg(palette.surface))
        .value_style(
            Style::default()
                .fg(Color::Black)
                .bg(palette.info)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(palette.text).bg(palette.surface))
        .style(Style::default().fg(palette.text).bg(palette.surface))
        .max(max_volume.ceil().max(1.0) as u64)
        .data(BarGroup::default().bars(&bars));
    frame.render_widget(chart, rows[1]);
}

fn render_watchlist(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let is_loading = app.binance_loading();
    let refresh_queued = app.binance_refresh_queued();
    let active = app.panel_focus() == PanelFocus::Table;
    let block = theme::panel_block(
        if app.is_english() {
            "Watchlist"
        } else {
            "Watchlist"
        },
        palette,
        palette.accent,
        active,
    );
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(4)])
        .margin(1)
        .split(inner);

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                badge(
                    if app.is_english() { "BOARD" } else { "BOARD" },
                    palette.accent,
                ),
                Span::raw(" "),
                Span::styled(
                    if is_loading && app.is_english() {
                        "quick pair switcher · syncing lane"
                    } else if is_loading {
                        "switcher pair cepat · lane syncing"
                    } else if refresh_queued && app.is_english() {
                        "quick pair switcher · queued refresh"
                    } else if refresh_queued {
                        "switcher pair cepat · refresh antre"
                    } else if app.is_english() {
                        "quick pair switcher"
                    } else {
                        "switcher pair cepat"
                    },
                    Style::default().fg(palette.muted),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "SYM",
                    Style::default()
                        .fg(palette.warn)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("      "),
                Span::styled(
                    "LAST",
                    Style::default()
                        .fg(palette.warn)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("         "),
                Span::styled(
                    "24H",
                    Style::default()
                        .fg(palette.warn)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ]),
        rows[0],
    );

    let tickers = &app.binance_dashboard().tickers;
    if tickers.is_empty() {
        render_empty(
            frame,
            rows[1],
            app,
            if is_loading && app.is_english() {
                "Pulling watchlist snapshot from Binance."
            } else if is_loading {
                "Mengambil snapshot watchlist dari Binance."
            } else if app.is_english() {
                "No watchlist snapshot yet."
            } else {
                "Belum ada snapshot watchlist."
            },
            palette,
        );
        return;
    }

    let lines = app
        .binance_watchlist()
        .iter()
        .enumerate()
        .map(|(idx, symbol)| {
            let ticker = tickers.iter().find(|item| item.symbol == *symbol);
            let selected = idx == app.binance_watchlist_selected_index();
            let loaded = *symbol == app.binance_selected_symbol();
            let marker = if loaded { ">" } else { "." };
            let change = ticker
                .map(|item| format_percent(item.price_change_percent))
                .unwrap_or_else(|| "--".into());
            let price = ticker
                .map(|item| format_decimal(item.last_price))
                .unwrap_or_else(|| "--".into());
            let style = if selected {
                Style::default()
                    .fg(palette.text)
                    .bg(palette.accent_soft)
                    .add_modifier(Modifier::BOLD)
            } else if loaded {
                Style::default().fg(palette.warn)
            } else {
                Style::default().fg(palette.text)
            };

            Line::from(vec![
                Span::styled(format!("{marker} "), style),
                Span::styled(
                    format!("{:<8}", symbol.trim_end_matches("USDT")),
                    style.add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
                Span::styled(format!("{:>12}", price), style),
                Span::raw(" "),
                Span::styled(
                    format!("{:>9}", change),
                    style.fg(
                        if ticker.is_some_and(|item| item.price_change_percent >= 0.0) {
                            palette.accent
                        } else {
                            palette.danger
                        },
                    ),
                ),
            ])
        })
        .collect::<Vec<_>>();

    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Left), rows[1]);
}

fn render_balances(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let is_loading = app.binance_loading();
    let active = app.panel_focus() == PanelFocus::TargetA;
    let block = theme::panel_block(
        if app.is_english() {
            "Spot Balances"
        } else {
            "Balance Spot"
        },
        palette,
        palette.warn,
        active,
    );
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(4)])
        .margin(1)
        .split(inner);

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                badge(
                    if app.is_english() {
                        "SPOT STACK"
                    } else {
                        "STACK SPOT"
                    },
                    palette.warn,
                ),
                Span::raw(" "),
                Span::styled(
                    if app.is_english() {
                        "free / locked / est. quote"
                    } else {
                        "free / locked / estimasi quote"
                    },
                    Style::default().fg(palette.muted),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "AST",
                    Style::default()
                        .fg(palette.warn)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("    "),
                Span::styled(
                    "TOTAL",
                    Style::default()
                        .fg(palette.warn)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("      "),
                Span::styled(
                    "FREE",
                    Style::default()
                        .fg(palette.warn)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("      "),
                Span::styled(
                    "LOCK",
                    Style::default()
                        .fg(palette.warn)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("    "),
                Span::styled(
                    "EST",
                    Style::default()
                        .fg(palette.warn)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ]),
        rows[0],
    );

    if app.binance_dashboard().balances.is_empty() {
        render_empty(
            frame,
            rows[1],
            app,
            if is_loading && app.is_english() {
                "Waiting for private account snapshot."
            } else if is_loading {
                "Menunggu snapshot akun privat."
            } else if app.is_english() {
                "No account balances available."
            } else {
                "Belum ada balance akun yang tersedia."
            },
            palette,
        );
        return;
    }

    let lines = app
        .binance_dashboard()
        .balances
        .iter()
        .take(8)
        .enumerate()
        .map(|(idx, balance)| {
            let selected = idx == app.binance_balance_selected_index();
            let style = if selected {
                Style::default()
                    .fg(palette.text)
                    .bg(palette.accent_soft)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(palette.text)
            };

            let estimate = balance
                .quote_value
                .map(|value| compact_number(value))
                .unwrap_or_else(|| "--".into());

            Line::from(vec![
                Span::styled(format!("{:<6}", balance.asset), style),
                Span::raw(" "),
                Span::styled(format!("{:>10}", format_decimal(balance.total)), style),
                Span::raw(" "),
                Span::styled(
                    format!("{:>10}", format_decimal(balance.free)),
                    Style::default().fg(palette.accent),
                ),
                Span::raw(" "),
                Span::styled(
                    format!("{:>8}", format_decimal(balance.locked)),
                    Style::default().fg(palette.info),
                ),
                Span::raw(" "),
                Span::styled(
                    format!("{:>10}", estimate),
                    Style::default().fg(palette.warn),
                ),
            ])
        })
        .collect::<Vec<_>>();

    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Left), rows[1]);
}

fn render_market_detail(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let is_loading = app.binance_loading();
    let dashboard = app.binance_dashboard();
    let ticker = dashboard.ticker_for(app.binance_selected_symbol());
    let latest = dashboard.klines.last();

    let block = theme::panel_block(
        if app.is_english() {
            "Market Detail"
        } else {
            "Detail Market"
        },
        palette,
        palette.danger,
        app.panel_focus() == PanelFocus::TargetB,
    );
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(2),
        ])
        .margin(1)
        .split(inner);

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                badge("DETAIL", palette.danger),
                Span::raw(" "),
                Span::styled(
                    app.binance_selected_symbol(),
                    Style::default()
                        .fg(palette.text)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("   "),
                Span::styled(
                    app.binance_selected_interval(),
                    Style::default().fg(palette.warn),
                ),
                Span::raw("   "),
                Span::styled(
                    if is_loading {
                        "SYNCING"
                    } else if ticker.is_some_and(|item| item.price_change_percent >= 0.0) {
                        "BULL TILT"
                    } else if ticker.is_some() {
                        "BEAR TILT"
                    } else {
                        "NO FEED"
                    },
                    Style::default().fg(if is_loading {
                        palette.info
                    } else if ticker.is_some_and(|item| item.price_change_percent >= 0.0) {
                        palette.accent
                    } else {
                        palette.danger
                    }),
                ),
            ]),
            Line::from(vec![Span::styled(
                trend_tape(
                    &app.binance_dashboard()
                        .klines
                        .iter()
                        .map(|point| point.close)
                        .collect::<Vec<_>>(),
                ),
                Style::default().fg(palette.info),
            )]),
        ]),
        rows[0],
    );

    frame.render_widget(
        Paragraph::new(theme::key_value_line(
            palette,
            if app.is_english() { "Last" } else { "Terakhir" },
            &ticker
                .map(|item| format_decimal(item.last_price))
                .unwrap_or_else(|| "--".into()),
            palette.warn,
        )),
        rows[1],
    );
    frame.render_widget(
        Paragraph::new(theme::key_value_line(
            palette,
            "24H",
            &ticker
                .map(|item| format_percent(item.price_change_percent))
                .unwrap_or_else(|| "--".into()),
            if ticker.is_some_and(|item| item.price_change_percent >= 0.0) {
                palette.accent
            } else {
                palette.danger
            },
        )),
        rows[2],
    );
    frame.render_widget(
        Paragraph::new(theme::key_value_line(
            palette,
            if app.is_english() { "Range" } else { "Range" },
            &ticker
                .map(|item| {
                    format!(
                        "{} - {}",
                        format_decimal(item.low_price),
                        format_decimal(item.high_price)
                    )
                })
                .unwrap_or_else(|| "--".into()),
            palette.info,
        )),
        rows[3],
    );
    frame.render_widget(
        Paragraph::new(theme::key_value_line(
            palette,
            if app.is_english() { "Candle" } else { "Candle" },
            &latest
                .map(|item| {
                    format!(
                        "O {} H {} L {} C {}",
                        format_decimal(item.open),
                        format_decimal(item.high),
                        format_decimal(item.low),
                        format_decimal(item.close)
                    )
                })
                .unwrap_or_else(|| "--".into()),
            palette.text,
        )),
        rows[4],
    );
    frame.render_widget(
        Paragraph::new(theme::key_value_line(
            palette,
            if app.is_english() { "Volume" } else { "Volume" },
            &ticker
                .map(|item| compact_number(item.volume))
                .unwrap_or_else(|| "--".into()),
            palette.accent,
        )),
        rows[5],
    );

    let detail = vec![
        Line::from(vec![
            Span::styled(
                if app.is_english() {
                    "Updated: "
                } else {
                    "Update: "
                },
                Style::default().fg(palette.muted),
            ),
            Span::styled(
                dashboard
                    .last_updated
                    .clone()
                    .unwrap_or_else(|| "--".into()),
                Style::default().fg(palette.text),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                if app.is_english() {
                    "Assets: "
                } else {
                    "Aset: "
                },
                Style::default().fg(palette.muted),
            ),
            Span::styled(
                dashboard.balances.len().to_string(),
                Style::default().fg(palette.warn),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                if app.is_english() {
                    "Wallet: "
                } else {
                    "Wallet: "
                },
                Style::default().fg(palette.muted),
            ),
            Span::styled(
                dashboard
                    .wallet_estimate_usdt
                    .map(|value| format!("{} USDT", compact_number(value)))
                    .unwrap_or_else(|| "--".into()),
                Style::default().fg(palette.info),
            ),
        ]),
    ];
    frame.render_widget(Paragraph::new(detail), rows[6]);
}

fn render_account_panel(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let is_loading = app.binance_loading();
    let account_message = if is_loading {
        if app.is_english() {
            "Binance worker is fetching public and account lanes without blocking the UI."
                .to_string()
        } else {
            "Worker Binance sedang mengambil jalur publik dan akun tanpa memblok UI.".to_string()
        }
    } else {
        app.binance_dashboard()
            .account_state
            .detail(app.is_english())
    };
    let block = theme::panel_block(
        if app.is_english() {
            "Account Status"
        } else {
            "Status Akun"
        },
        palette,
        palette.info,
        false,
    );
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(4)])
        .margin(1)
        .split(inner);

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                badge(
                    if is_loading {
                        if app.is_english() {
                            "Syncing"
                        } else {
                            "Syncing"
                        }
                    } else {
                        app.binance_dashboard()
                            .account_state
                            .title(app.is_english())
                    },
                    if is_loading {
                        palette.info
                    } else {
                        match app.binance_dashboard().account_state {
                            crate::binance::AccountState::Connected { .. } => palette.accent,
                            crate::binance::AccountState::MissingCredentials => palette.warn,
                            crate::binance::AccountState::Error(_) => palette.danger,
                        }
                    },
                ),
                Span::raw(" "),
                Span::styled(
                    if app.is_english() {
                        "private spot snapshot"
                    } else {
                        "snapshot spot privat"
                    },
                    Style::default().fg(palette.muted),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    if app.is_english() {
                        "Signal "
                    } else {
                        "Signal "
                    },
                    Style::default().fg(palette.muted),
                ),
                Span::styled(account_tape(app), Style::default().fg(palette.info)),
            ]),
        ]),
        rows[0],
    );

    let info = vec![
        Line::styled(
            account_message,
            Style::default().fg(if is_loading {
                palette.info
            } else {
                palette.text
            }),
        ),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                if app.is_english() { "Flow: " } else { "Alur: " },
                Style::default().fg(palette.muted),
            ),
            Span::styled(
                if app.is_english() {
                    "Watchlist -> Price -> Volume -> Balances -> Detail"
                } else {
                    "Watchlist -> Harga -> Volume -> Balance -> Detail"
                },
                Style::default().fg(palette.info),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                if app.is_english() { "Tip: " } else { "Tip: " },
                Style::default().fg(palette.muted),
            ),
            Span::styled(
                if app.is_english() {
                    "Keep the real key in .env only. Public data can still run without account access."
                } else {
                    "Simpan key asli di .env saja. Data publik tetap bisa jalan tanpa akses akun."
                },
                Style::default().fg(palette.warn),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                if app.is_english() {
                    "Last status: "
                } else {
                    "Status terakhir: "
                },
                Style::default().fg(palette.muted),
            ),
            Span::styled(
                app.status().to_string(),
                Style::default().fg(palette.accent),
            ),
        ]),
    ];
    frame.render_widget(Paragraph::new(info).alignment(Alignment::Left), rows[1]);
}

fn render_empty(frame: &mut Frame, area: Rect, app: &App, text: &str, palette: ThemePalette) {
    let tone = if app.binance_loading() {
        palette.info
    } else if app.binance_refresh_queued() {
        palette.warn
    } else {
        palette.muted
    };

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(vec![Span::styled(
                if app.binance_loading() && app.is_english() {
                    "sync lane active"
                } else if app.binance_loading() {
                    "lane sync aktif"
                } else if app.binance_refresh_queued() && app.is_english() {
                    "refresh queue armed"
                } else if app.binance_refresh_queued() {
                    "antrian refresh siap"
                } else if app.is_english() {
                    "data lane idle"
                } else {
                    "jalur data idle"
                },
                Style::default().fg(tone).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(text),
            Line::from(""),
            Line::from(if app.binance_loading() && app.is_english() {
                "the desk stays interactive while the worker refreshes"
            } else if app.binance_loading() {
                "desk tetap interaktif saat worker melakukan refresh"
            } else if app.binance_refresh_queued() && app.is_english() {
                "a second refresh is queued after the current fetch"
            } else if app.binance_refresh_queued() {
                "refresh kedua akan jalan setelah fetch sekarang selesai"
            } else if app.is_english() {
                "refresh the pair or wait for the feed"
            } else {
                "refresh pair atau tunggu feed"
            }),
        ])
        .alignment(Alignment::Center)
        .style(Style::default().fg(tone)),
        area,
    );
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

fn short_time(timestamp_ms: i64, interval: &str) -> String {
    let Some(datetime) = Local.timestamp_millis_opt(timestamp_ms).single() else {
        return "--".into();
    };

    if interval.ends_with('d') {
        datetime.format("%m-%d").to_string()
    } else {
        datetime.format("%H:%M").to_string()
    }
}

fn trend_tape(values: &[f64]) -> String {
    if values.is_empty() {
        return "........".into();
    }

    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let spread = (max - min).max(0.000_001);
    let glyphs = ['.', ':', '-', '=', '+', '*', '#', '@'];

    values
        .iter()
        .rev()
        .take(20)
        .copied()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|value| {
            let ratio = ((value - min) / spread).clamp(0.0, 0.999);
            glyphs[(ratio * glyphs.len() as f64) as usize]
        })
        .collect()
}

fn account_tape(app: &App) -> &'static str {
    if app.binance_loading() {
        "[worker syncing account lane]"
    } else if app.binance_refresh_queued() {
        "[next refresh queued]"
    } else {
        match app.binance_dashboard().account_state {
            crate::binance::AccountState::Connected { .. } => "[wallet link stable]",
            crate::binance::AccountState::MissingCredentials => "[waiting credentials]",
            crate::binance::AccountState::Error(_) => "[account route blocked]",
        }
    }
}
