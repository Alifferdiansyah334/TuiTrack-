use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

use super::App;
use crate::state::{AppScreen, HomeMenu, LanguagePreset, Mode, ThemePreset};

impl App {
    pub(crate) fn handle_home_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
            KeyCode::Char('t') => self.open_theme_selector(),
            KeyCode::Char('l') => self.open_language_selector(),
            KeyCode::Up | KeyCode::Char('k') => {
                if self.home_selected == 0 {
                    self.home_selected = HomeMenu::ALL.len() - 1;
                } else {
                    self.home_selected -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.home_selected = (self.home_selected + 1) % HomeMenu::ALL.len();
            }
            KeyCode::Enter => match self.home_menu() {
                HomeMenu::ExpenseTracker => {
                    self.screen = AppScreen::ExpenseTracker;
                    self.status = if self.is_english() {
                        "Entered Expense Tracker.".into()
                    } else {
                        "Masuk ke Expense Tracker.".into()
                    };
                }
                HomeMenu::WorkTracker => {
                    self.screen = AppScreen::WorkTracker;
                    self.mode = Mode::Normal;
                    self.panel_focus = crate::state::PanelFocus::Table;
                    self.clamp_selection();
                    self.status = if self.is_english() {
                        "Entered Work Tracker.".into()
                    } else {
                        "Masuk ke Work Tracker.".into()
                    };
                }
                HomeMenu::SecretNotes => {
                    self.screen = AppScreen::SecretNotes;
                    self.mode = Mode::Normal;
                    self.clamp_selection();
                    self.status = if self.is_english() {
                        "Entered Secret Notes vault.".into()
                    } else {
                        "Masuk ke vault Secret Notes.".into()
                    };
                }
                HomeMenu::BinanceTracker => {
                    self.open_binance_tracker()?;
                }
            },
            _ => {}
        }
        Ok(false)
    }

    pub(crate) fn handle_theme_selector_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.status = self.status_cancelled("theme_selector");
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.theme_selected == 0 {
                    self.theme_selected = ThemePreset::ALL.len() - 1;
                } else {
                    self.theme_selected -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.theme_selected = (self.theme_selected + 1) % ThemePreset::ALL.len();
            }
            KeyCode::Enter => {
                self.theme_preset = ThemePreset::ALL[self.theme_selected];
                self.persist()?;
                self.mode = Mode::Normal;
                self.status = format!(
                    "{} {}.",
                    if self.is_english() {
                        "Theme changed to"
                    } else {
                        "Theme diganti ke"
                    },
                    self.theme_preset.label()
                );
            }
            _ => {}
        }
        Ok(false)
    }

    pub(crate) fn handle_language_selector_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.status = self.status_cancelled("language_selector");
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.language_selected == 0 {
                    self.language_selected = LanguagePreset::ALL.len() - 1;
                } else {
                    self.language_selected -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.language_selected = (self.language_selected + 1) % LanguagePreset::ALL.len();
            }
            KeyCode::Enter => {
                self.language_preset = LanguagePreset::ALL[self.language_selected];
                self.persist()?;
                self.mode = Mode::Normal;
                self.status = format!(
                    "{} {}.",
                    if self.is_english() {
                        "Language changed to"
                    } else {
                        "Bahasa diganti ke"
                    },
                    self.language_preset.label()
                );
            }
            _ => {}
        }
        Ok(false)
    }
}
