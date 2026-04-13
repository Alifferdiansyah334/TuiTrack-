use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::App;
use crate::state::{ConfirmChoice, Mode, PanelFocus, TargetFormField, TargetMode};

impl App {
    pub(crate) fn move_panel_focus(&mut self, delta: isize) {
        let current = PanelFocus::ORDER
            .iter()
            .position(|focus| *focus == self.panel_focus)
            .unwrap_or(0) as isize;
        let len = PanelFocus::ORDER.len() as isize;
        let next = (current + delta).rem_euclid(len) as usize;
        self.panel_focus = PanelFocus::ORDER[next];
    }

    pub(crate) fn handle_add_target_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.status = self.status_cancelled("target_form");
            }
            KeyCode::Backspace => match self.target_form.focus {
                TargetFormField::Title => {
                    self.target_form.title.pop();
                }
                TargetFormField::Amount => {
                    self.target_form.amount.pop();
                }
                TargetFormField::Kind => {}
            },
            KeyCode::Left => {
                if self.target_form.focus == TargetFormField::Kind {
                    self.target_form.mode = TargetMode::Saving;
                }
            }
            KeyCode::Right => {
                if self.target_form.focus == TargetFormField::Kind {
                    self.target_form.mode = TargetMode::TotalBalance;
                }
            }
            KeyCode::Up | KeyCode::BackTab => {
                self.target_form.focus = self.target_form.focus.prev()
            }
            KeyCode::Down | KeyCode::Tab => self.target_form.focus = self.target_form.focus.next(),
            KeyCode::Enter => {
                if self.target_form.focus == TargetFormField::Amount {
                    self.submit_target_form()?;
                } else {
                    self.target_form.focus = self.target_form.focus.next();
                }
            }
            KeyCode::Char(c)
                if (key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT)
                    && self.target_form.focus != TargetFormField::Kind =>
            {
                match self.target_form.focus {
                    TargetFormField::Title => self.target_form.title.push(c),
                    TargetFormField::Amount => self.target_form.amount.push(c),
                    TargetFormField::Kind => {}
                }
            }
            _ => {}
        }
        Ok(false)
    }

    pub(crate) fn open_target_delete_confirm(&mut self) {
        if let Some((target_id, target_title)) = self
            .focused_target()
            .map(|target| (target.id, target.title.clone()))
        {
            self.pending_delete_target_id = Some(target_id);
            self.confirm_choice = ConfirmChoice::No;
            self.mode = Mode::ConfirmTargetDelete;
            self.status = if self.is_english() {
                format!("Delete target '{}'?", target_title)
            } else {
                format!("Hapus target '{}'?", target_title)
            };
        } else {
            self.status = if self.is_english() {
                "No target on this chart.".into()
            } else {
                "Tidak ada target pada chart ini.".into()
            };
        }
    }
}
