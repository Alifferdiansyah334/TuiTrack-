use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::App;
use crate::state::{AppScreen, Mode, PanelFocus, WorkFormField};

impl App {
    pub(crate) fn handle_work_normal_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Char('q') => {
                self.mode = Mode::Normal;
                self.screen = AppScreen::Home;
                self.status = if self.is_english() {
                    "Back to launcher.".into()
                } else {
                    "Kembali ke launcher.".into()
                };
            }
            KeyCode::Down => {
                if self.panel_focus == PanelFocus::Table {
                    self.move_selection(1);
                }
            }
            KeyCode::Up => {
                if self.panel_focus == PanelFocus::Table {
                    self.move_selection(-1);
                }
            }
            KeyCode::Char('j') => self.move_panel_focus(1),
            KeyCode::Char('k') => self.move_panel_focus(-1),
            KeyCode::Home => {
                if self.panel_focus == PanelFocus::Table {
                    self.set_selected(0);
                }
            }
            KeyCode::End | KeyCode::Char('G') => {
                if self.panel_focus == PanelFocus::Table {
                    self.set_selected(self.current_len().saturating_sub(1));
                }
            }
            KeyCode::Char('/') | KeyCode::Char('f') => {
                self.mode = Mode::Filter;
                self.status = if self.is_english() {
                    "Filter active for Work Tracker.".into()
                } else {
                    "Filter aktif untuk Work Tracker.".into()
                };
            }
            KeyCode::Char('c') => {
                self.filter.clear();
                self.clamp_selection();
                self.status = if self.is_english() {
                    "Filter cleared.".into()
                } else {
                    "Filter dibersihkan.".into()
                };
            }
            KeyCode::Char('t') => self.open_theme_selector(),
            KeyCode::Char('l') => self.open_language_selector(),
            KeyCode::Char('r') => self.open_reset_all(),
            KeyCode::Char('a') => self.open_work_form(),
            KeyCode::Char('x') => self.toggle_selected_work_completion()?,
            KeyCode::Enter => {
                if self.panel_focus == PanelFocus::Table {
                    self.toggle_selected_work_completion()?;
                }
            }
            KeyCode::Char('d') => self.delete_selected_work()?,
            _ => {}
        }

        self.clamp_selection();
        Ok(false)
    }

    pub(crate) fn handle_add_work_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.status = self.status_cancelled("work_form");
            }
            KeyCode::Backspace => {
                self.work_form.current_value_mut().pop();
            }
            KeyCode::Up | KeyCode::BackTab => self.work_form.focus = self.work_form.focus.prev(),
            KeyCode::Down | KeyCode::Tab => self.work_form.focus = self.work_form.focus.next(),
            KeyCode::Enter => {
                if self.work_form.focus == WorkFormField::Time {
                    self.submit_work_form()?;
                } else {
                    self.work_form.focus = self.work_form.focus.next();
                }
            }
            KeyCode::Char(c)
                if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
            {
                self.work_form.current_value_mut().push(c);
            }
            _ => {}
        }
        Ok(false)
    }
}
