use anyhow::Result;

use super::App;
use crate::state::{Mode, PanelFocus, ResetForm};

impl App {
    pub(crate) fn open_reset_all(&mut self) {
        self.mode = Mode::ResetAll;
        self.reset_form = ResetForm::default();
        self.status = if self.is_english() {
            "Type /resetall to delete all data.".into()
        } else {
            "Ketik /resetall untuk hapus seluruh data.".into()
        };
    }

    pub(crate) fn submit_reset_all(&mut self) -> Result<()> {
        if self.reset_form.confirmation.trim() != "/resetall" {
            anyhow::bail!(if self.is_english() {
                "Type /resetall exactly to continue"
            } else {
                "Ketik persis /resetall untuk melanjutkan"
            });
        }

        self.expenses.clear();
        self.savings.clear();
        self.earnings.clear();
        self.work_tasks.clear();
        self.secret_notes.clear();
        self.unlocked_notes.clear();
        self.targets.items.clear();
        self.balance.enabled = false;
        self.balance.amount = 0.0;
        self.filter.clear();
        self.expense_selected = 0;
        self.saving_selected = 0;
        self.earning_selected = 0;
        self.secret_selected = 0;
        self.target_page = 0;
        self.panel_focus = PanelFocus::Table;
        self.pending_delete_target_id = None;
        self.pending_unlock_note_id = None;
        self.editing_secret_note_id = None;
        self.confirm_choice = crate::state::ConfirmChoice::No;
        self.mode = Mode::Normal;
        self.reset_form = ResetForm::default();
        self.next_expense_id = 1;
        self.next_saving_id = 1;
        self.next_earning_id = 1;
        self.next_work_id = 1;
        self.next_secret_note_id = 1;
        self.next_target_id = 1;
        self.persist()?;
        self.status = if self.is_english() {
            "All data was reset successfully.".into()
        } else {
            "Semua data berhasil direset.".into()
        };
        Ok(())
    }
}
