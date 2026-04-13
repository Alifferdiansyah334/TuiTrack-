use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::App;
use crate::{
    models::TrackerMode,
    state::{
        AppScreen, BalanceFormField, ConfirmChoice, EarningForm, EarningFormField, ExpenseForm,
        ExpenseFormField, Mode, PanelFocus, SavingForm, SavingFormField,
    },
};

impl App {
    pub(crate) fn handle_normal_key(&mut self, key: KeyEvent) -> Result<bool> {
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
                self.status = format!(
                    "{} {}.",
                    if self.is_english() {
                        "Filter active for mode"
                    } else {
                        "Filter aktif untuk mode"
                    },
                    self.tracker_mode.label(self.language_preset)
                );
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
            KeyCode::Char('1') => self.switch_tracker_mode(TrackerMode::Expense),
            KeyCode::Char('2') => self.switch_tracker_mode(TrackerMode::Saving),
            KeyCode::Char('3') => self.switch_tracker_mode(TrackerMode::Earning),
            KeyCode::Char('t') => self.open_theme_selector(),
            KeyCode::Char('l') => self.open_language_selector(),
            KeyCode::Char('b') => self.open_balance_form(),
            KeyCode::Char('g') => self.open_target_form(),
            KeyCode::Char('r') => self.open_reset_all(),
            KeyCode::Char('p') | KeyCode::Char('P') => self.next_target_page(),
            KeyCode::Enter => {
                if self.focused_target().is_some() {
                    self.open_target_delete_confirm();
                }
            }
            KeyCode::Char('m') | KeyCode::Tab => {
                let next = match self.tracker_mode {
                    TrackerMode::Expense => TrackerMode::Saving,
                    TrackerMode::Saving => TrackerMode::Earning,
                    TrackerMode::Earning => TrackerMode::Expense,
                };
                self.switch_tracker_mode(next);
            }
            KeyCode::Char('a') => match self.tracker_mode {
                TrackerMode::Expense => {
                    self.mode = Mode::AddExpense;
                    self.expense_form = ExpenseForm::default();
                    self.status = if self.is_english() {
                        "Add expense. Pick a category, then fill the details.".into()
                    } else {
                        "Tambah pengeluaran. Pilih kategori lalu isi detail.".into()
                    };
                }
                TrackerMode::Saving => {
                    self.mode = Mode::AddSaving;
                    self.saving_form = SavingForm::default();
                    self.status = if self.is_english() {
                        "Add saving entry.".into()
                    } else {
                        "Tambah data nabung.".into()
                    };
                }
                TrackerMode::Earning => {
                    self.mode = Mode::AddEarning;
                    self.earning_form = EarningForm::default();
                    self.status = if self.is_english() {
                        "Add earning entry.".into()
                    } else {
                        "Tambah data pemasukan.".into()
                    };
                }
            },
            KeyCode::Char('d') => self.delete_selected()?,
            _ => {}
        }

        self.clamp_selection();
        Ok(false)
    }

    pub(crate) fn handle_filter_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => {
                self.mode = Mode::Normal;
                self.status = if self.filter_is_active() {
                    format!(
                        "{}: {}",
                        if self.is_english() {
                            "Filter active"
                        } else {
                            "Filter aktif"
                        },
                        self.filter
                    )
                } else {
                    if self.is_english() {
                        "Filter cleared.".into()
                    } else {
                        "Filter dibersihkan.".into()
                    }
                };
            }
            KeyCode::Backspace => {
                self.filter.pop();
                self.clamp_selection();
            }
            KeyCode::Char(c)
                if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
            {
                self.filter.push(c);
                self.clamp_selection();
            }
            _ => {}
        }
        Ok(false)
    }

    pub(crate) fn handle_confirm_target_delete_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.pending_delete_target_id = None;
                self.confirm_choice = ConfirmChoice::No;
                self.status = self.status_cancelled("target_delete");
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('k') => {
                self.confirm_choice = ConfirmChoice::Yes;
            }
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('j') => {
                self.confirm_choice = ConfirmChoice::No;
            }
            KeyCode::Enter => {
                if self.confirm_choice == ConfirmChoice::Yes {
                    self.delete_pending_target()?;
                } else {
                    self.mode = Mode::Normal;
                    self.pending_delete_target_id = None;
                    self.status = if self.is_english() {
                        "Target was not deleted.".into()
                    } else {
                        "Target tidak dihapus.".into()
                    };
                }
            }
            _ => {}
        }
        Ok(false)
    }

    pub(crate) fn handle_add_balance_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.status = self.status_cancelled("balance");
            }
            KeyCode::Backspace => {
                if self.balance_form.focus == BalanceFormField::Amount {
                    self.balance_form.amount.pop();
                }
            }
            KeyCode::Left | KeyCode::Right | KeyCode::Char(' ') => {
                if self.balance_form.focus == BalanceFormField::Enabled {
                    self.balance_form.enabled = !self.balance_form.enabled;
                }
            }
            KeyCode::Up | KeyCode::BackTab => {
                self.balance_form.focus = self.balance_form.focus.prev()
            }
            KeyCode::Down | KeyCode::Tab => {
                self.balance_form.focus = self.balance_form.focus.next()
            }
            KeyCode::Enter => {
                if self.balance_form.focus == BalanceFormField::Amount {
                    self.submit_balance_form()?;
                } else {
                    self.balance_form.focus = self.balance_form.focus.next();
                }
            }
            KeyCode::Char(c)
                if (key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT)
                    && self.balance_form.focus == BalanceFormField::Amount =>
            {
                self.balance_form.amount.push(c);
            }
            _ => {}
        }
        Ok(false)
    }

    pub(crate) fn handle_add_expense_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.status = self.status_cancelled("expense_form");
            }
            KeyCode::Backspace => {
                if let Some(value) = self.expense_form.current_value_mut() {
                    value.pop();
                }
            }
            KeyCode::Left if self.expense_form.focus == ExpenseFormField::Category => {
                self.expense_form.prev_category()
            }
            KeyCode::Right if self.expense_form.focus == ExpenseFormField::Category => {
                self.expense_form.next_category()
            }
            KeyCode::Up | KeyCode::BackTab => {
                self.expense_form.focus = self.expense_form.focus.prev()
            }
            KeyCode::Down | KeyCode::Tab => {
                self.expense_form.focus = self.expense_form.focus.next()
            }
            KeyCode::Enter => {
                if self.expense_form.focus == ExpenseFormField::Amount {
                    self.submit_expense_form()?;
                } else {
                    self.expense_form.focus = self.expense_form.focus.next();
                }
            }
            KeyCode::Char(c)
                if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
            {
                if let Some(value) = self.expense_form.current_value_mut() {
                    value.push(c);
                }
            }
            _ => {}
        }
        Ok(false)
    }

    pub(crate) fn handle_add_saving_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.status = self.status_cancelled("saving_form");
            }
            KeyCode::Backspace => {
                self.saving_form.current_value_mut().pop();
            }
            KeyCode::Up | KeyCode::BackTab => {
                self.saving_form.focus = self.saving_form.focus.prev()
            }
            KeyCode::Down | KeyCode::Tab => self.saving_form.focus = self.saving_form.focus.next(),
            KeyCode::Enter => {
                if self.saving_form.focus == SavingFormField::Amount {
                    self.submit_saving_form()?;
                } else {
                    self.saving_form.focus = self.saving_form.focus.next();
                }
            }
            KeyCode::Char(c)
                if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
            {
                self.saving_form.current_value_mut().push(c);
            }
            _ => {}
        }
        Ok(false)
    }

    pub(crate) fn handle_add_earning_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.status = self.status_cancelled("earning_form");
            }
            KeyCode::Backspace => {
                self.earning_form.current_value_mut().pop();
            }
            KeyCode::Up | KeyCode::BackTab => {
                self.earning_form.focus = self.earning_form.focus.prev()
            }
            KeyCode::Down | KeyCode::Tab => {
                self.earning_form.focus = self.earning_form.focus.next()
            }
            KeyCode::Enter => {
                if self.earning_form.focus == EarningFormField::Amount {
                    self.submit_earning_form()?;
                } else {
                    self.earning_form.focus = self.earning_form.focus.next();
                }
            }
            KeyCode::Char(c)
                if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
            {
                self.earning_form.current_value_mut().push(c);
            }
            _ => {}
        }
        Ok(false)
    }
}
