use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

use super::App;
use crate::state::{AppScreen, PanelFocus};

impl App {
    pub(crate) fn handle_binance_normal_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Char('q') => {
                self.mode = crate::state::Mode::Normal;
                self.screen = AppScreen::Home;
                self.status = if self.is_english() {
                    "Back to launcher.".into()
                } else {
                    "Kembali ke launcher.".into()
                };
            }
            KeyCode::Down => {
                if matches!(self.panel_focus, PanelFocus::Table | PanelFocus::TargetA) {
                    self.move_selection(1);
                }
            }
            KeyCode::Up => {
                if matches!(self.panel_focus, PanelFocus::Table | PanelFocus::TargetA) {
                    self.move_selection(-1);
                }
            }
            KeyCode::Char('j') => self.move_panel_focus(1),
            KeyCode::Char('k') => self.move_panel_focus(-1),
            KeyCode::Left => self.cycle_binance_interval(-1)?,
            KeyCode::Right => self.cycle_binance_interval(1)?,
            KeyCode::Char('u') | KeyCode::Char('r') => self.refresh_binance_dashboard()?,
            KeyCode::Enter => {
                if self.panel_focus == PanelFocus::Table {
                    self.activate_selected_binance_symbol()?;
                }
            }
            KeyCode::Char('t') => self.open_theme_selector(),
            KeyCode::Char('s') => {
                self.panel_focus = PanelFocus::Table;
                self.status = if self.is_english() {
                    "Focus moved to watchlist.".into()
                } else {
                    "Fokus pindah ke watchlist.".into()
                };
            }
            KeyCode::Char('b') => {
                self.panel_focus = PanelFocus::TargetA;
                self.status = if self.is_english() {
                    "Focus moved to balances.".into()
                } else {
                    "Fokus pindah ke balance.".into()
                };
            }
            KeyCode::Char('l') => self.open_language_selector(),
            _ => {}
        }

        self.clamp_selection();
        Ok(false)
    }
}
