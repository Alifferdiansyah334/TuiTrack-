use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::App;
use crate::state::{AppScreen, Mode, SecretNoteFormField};

impl App {
    pub(crate) fn handle_secret_normal_key(&mut self, key: KeyEvent) -> Result<bool> {
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
            KeyCode::Down => self.move_selection(1),
            KeyCode::Up => self.move_selection(-1),
            KeyCode::Home => self.set_selected(0),
            KeyCode::End | KeyCode::Char('G') => {
                self.set_selected(self.current_len().saturating_sub(1));
            }
            KeyCode::Char('/') | KeyCode::Char('f') => {
                self.mode = Mode::Filter;
                self.status = if self.is_english() {
                    "Filter active for Secret Notes.".into()
                } else {
                    "Filter aktif untuk Secret Notes.".into()
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
            KeyCode::Char('a') => self.open_secret_note_form(),
            KeyCode::Char('e') => self.open_edit_secret_note_form(),
            KeyCode::Char('u') | KeyCode::Enter => self.open_unlock_note_form(),
            KeyCode::Char('d') => self.delete_selected_secret_note()?,
            KeyCode::Char('r') => self.open_reset_all(),
            KeyCode::Char('t') => self.open_theme_selector(),
            KeyCode::Char('l') => self.open_language_selector(),
            _ => {}
        }

        self.clamp_selection();
        Ok(false)
    }

    pub(crate) fn handle_add_secret_note_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.status = self.status_cancelled("secret_note_form");
            }
            KeyCode::Backspace => {
                self.secret_note_form.current_value_mut().pop();
            }
            KeyCode::Up | KeyCode::BackTab => {
                self.secret_note_form.focus = self.secret_note_form.focus.prev()
            }
            KeyCode::Down | KeyCode::Tab => {
                self.secret_note_form.focus = self.secret_note_form.focus.next()
            }
            KeyCode::Enter => {
                if self.secret_note_form.focus == SecretNoteFormField::Passkey {
                    self.submit_secret_note_form()?;
                } else {
                    self.secret_note_form.focus = self.secret_note_form.focus.next();
                }
            }
            KeyCode::Char(c)
                if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
            {
                self.secret_note_form.current_value_mut().push(c);
            }
            _ => {}
        }
        Ok(false)
    }

    pub(crate) fn handle_unlock_secret_note_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.pending_unlock_note_id = None;
                self.unlock_note_form.passkey.clear();
                self.status = self.status_cancelled("unlock_note_form");
            }
            KeyCode::Backspace => {
                self.unlock_note_form.passkey.pop();
            }
            KeyCode::Enter => {
                self.submit_unlock_note_form()?;
            }
            KeyCode::Char(c)
                if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
            {
                self.unlock_note_form.passkey.push(c);
            }
            _ => {}
        }
        Ok(false)
    }

    pub(crate) fn handle_edit_secret_note_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.editing_secret_note_id = None;
                self.secret_note_form = crate::state::SecretNoteForm::default();
                self.status = self.status_cancelled("edit_secret_note_form");
            }
            KeyCode::Backspace => {
                self.secret_note_form.current_value_mut().pop();
            }
            KeyCode::Up | KeyCode::BackTab => {
                self.secret_note_form.focus = self.secret_note_form.focus.prev()
            }
            KeyCode::Down | KeyCode::Tab => {
                self.secret_note_form.focus = self.secret_note_form.focus.next()
            }
            KeyCode::Enter => {
                if self.secret_note_form.focus == SecretNoteFormField::Passkey {
                    self.submit_edit_secret_note_form()?;
                } else {
                    self.secret_note_form.focus = self.secret_note_form.focus.next();
                }
            }
            KeyCode::Char(c)
                if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
            {
                self.secret_note_form.current_value_mut().push(c);
            }
            _ => {}
        }
        Ok(false)
    }
}
