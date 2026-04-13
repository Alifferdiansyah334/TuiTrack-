use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::App;
use crate::state::Mode;

impl App {
    pub(crate) fn handle_reset_all_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.reset_form.confirmation.clear();
                self.status = self.status_cancelled("reset");
            }
            KeyCode::Backspace => {
                self.reset_form.confirmation.pop();
            }
            KeyCode::Enter => {
                self.submit_reset_all()?;
            }
            KeyCode::Char(c)
                if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
            {
                self.reset_form.confirmation.push(c);
            }
            _ => {}
        }
        Ok(false)
    }
}
