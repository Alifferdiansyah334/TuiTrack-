use anyhow::Result;

use super::App;
use crate::{
    binance::{self, INTERVALS, WATCHLIST},
    state::{AppScreen, Mode, PanelFocus},
};

impl App {
    pub(crate) fn open_binance_tracker(&mut self) -> Result<()> {
        self.screen = AppScreen::BinanceTracker;
        self.mode = Mode::Normal;
        self.panel_focus = PanelFocus::Table;
        self.binance_watchlist_selected = self.binance_symbol_selected.min(WATCHLIST.len() - 1);
        self.request_binance_refresh();
        Ok(())
    }

    pub(crate) fn refresh_binance_dashboard(&mut self) -> Result<()> {
        self.request_binance_refresh();
        Ok(())
    }

    pub(crate) fn activate_selected_binance_symbol(&mut self) -> Result<()> {
        self.binance_symbol_selected = self.binance_watchlist_selected.min(WATCHLIST.len() - 1);
        self.refresh_binance_dashboard()
    }

    pub(crate) fn cycle_binance_interval(&mut self, delta: isize) -> Result<()> {
        let len = INTERVALS.len() as isize;
        let current = self.binance_interval_selected as isize;
        self.binance_interval_selected = (current + delta).rem_euclid(len) as usize;
        self.refresh_binance_dashboard()
    }

    pub(crate) fn request_binance_refresh(&mut self) {
        if self.binance_loading {
            self.binance_refresh_queued = true;
            self.status = if self.is_english() {
                format!(
                    "Binance sync queued for {} / {}.",
                    self.binance_selected_symbol(),
                    self.binance_selected_interval()
                )
            } else {
                format!(
                    "Sync Binance diantrikan untuk {} / {}.",
                    self.binance_selected_symbol(),
                    self.binance_selected_interval()
                )
            };
            return;
        }

        self.spawn_binance_refresh();
    }

    pub(crate) fn spawn_binance_refresh(&mut self) {
        let symbol = self.binance_selected_symbol().to_string();
        let interval = self.binance_selected_interval().to_string();
        self.binance_loading = true;
        self.binance_refresh_queued = false;
        self.status = if self.is_english() {
            format!("Syncing Binance data for {} / {}...", symbol, interval)
        } else {
            format!(
                "Sedang sync data Binance untuk {} / {}...",
                symbol, interval
            )
        };
        self.binance_fetch_handle = Some(std::thread::spawn(move || {
            binance::fetch_dashboard(&symbol, &interval).map_err(|err| err.to_string())
        }));
    }
}
