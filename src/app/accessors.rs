use super::App;
use crate::{
    binance::{BinanceDashboard, INTERVALS, WATCHLIST},
    models::TrackerMode,
    state::{
        BalanceForm, BudgetTarget, ConfirmChoice, EarningForm, HomeMenu, LanguagePreset, Mode,
        PanelFocus, ResetForm, SavingForm, SecretNoteForm, TargetForm, ThemePreset, UnlockNoteForm,
        WorkForm,
    },
};

impl App {
    pub fn tracker_mode(&self) -> TrackerMode {
        self.tracker_mode
    }

    pub fn screen(&self) -> crate::state::AppScreen {
        self.screen
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn filter(&self) -> &str {
        &self.filter
    }

    pub fn filter_is_active(&self) -> bool {
        !self.filter.trim().is_empty()
    }

    pub fn balance_enabled(&self) -> bool {
        self.balance.enabled
    }

    pub fn balance_amount(&self) -> f64 {
        self.balance.amount
    }

    pub fn targets(&self) -> &[BudgetTarget] {
        &self.targets.items
    }

    pub fn target_page(&self) -> usize {
        self.target_page
    }

    pub fn target_page_count(&self) -> usize {
        self.targets.items.len().max(1).div_ceil(2)
    }

    pub fn panel_focus(&self) -> PanelFocus {
        self.panel_focus
    }

    pub fn confirm_choice(&self) -> ConfirmChoice {
        self.confirm_choice
    }

    pub fn celebration_active(&self) -> bool {
        self.celebration_frames_left > 0
    }

    pub fn celebration_target_title(&self) -> &str {
        &self.celebration_target_title
    }

    pub fn celebration_success(&self) -> bool {
        self.celebration_success
    }

    pub fn work_delete_active(&self) -> bool {
        self.work_delete_frames_left > 0
    }

    pub fn work_delete_task_title(&self) -> &str {
        &self.work_delete_task_title
    }

    pub fn work_delete_was_completed(&self) -> bool {
        self.work_delete_was_completed
    }

    pub fn theme_preset(&self) -> ThemePreset {
        self.theme_preset
    }

    pub fn theme_selected_index(&self) -> usize {
        self.theme_selected
    }

    pub fn language_preset(&self) -> LanguagePreset {
        self.language_preset
    }

    pub fn language_selected_index(&self) -> usize {
        self.language_selected
    }

    pub fn animation_tick(&self) -> u64 {
        self.animation_tick
    }

    pub fn expense_form(&self) -> &crate::state::ExpenseForm {
        &self.expense_form
    }

    pub fn balance_form(&self) -> &BalanceForm {
        &self.balance_form
    }

    pub fn work_form(&self) -> &WorkForm {
        &self.work_form
    }

    pub fn secret_note_form(&self) -> &SecretNoteForm {
        &self.secret_note_form
    }

    pub fn unlock_note_form(&self) -> &UnlockNoteForm {
        &self.unlock_note_form
    }

    pub fn target_form(&self) -> &TargetForm {
        &self.target_form
    }

    pub fn reset_form(&self) -> &ResetForm {
        &self.reset_form
    }

    pub fn saving_form(&self) -> &SavingForm {
        &self.saving_form
    }

    pub fn earning_form(&self) -> &EarningForm {
        &self.earning_form
    }

    pub fn expense_selected_index(&self) -> usize {
        self.expense_selected
    }

    pub fn saving_selected_index(&self) -> usize {
        self.saving_selected
    }

    pub fn earning_selected_index(&self) -> usize {
        self.earning_selected
    }

    pub fn home_selected_index(&self) -> usize {
        self.home_selected
    }

    pub fn binance_watchlist_selected_index(&self) -> usize {
        self.binance_watchlist_selected
    }

    pub fn binance_balance_selected_index(&self) -> usize {
        self.binance_balance_selected
    }

    pub fn binance_dashboard(&self) -> &BinanceDashboard {
        &self.binance_dashboard
    }

    pub fn binance_loading(&self) -> bool {
        self.binance_loading
    }

    pub fn binance_refresh_queued(&self) -> bool {
        self.binance_refresh_queued
    }

    pub fn binance_watchlist(&self) -> &'static [&'static str] {
        &WATCHLIST
    }

    pub fn binance_selected_symbol(&self) -> &'static str {
        WATCHLIST
            .get(self.binance_symbol_selected)
            .copied()
            .unwrap_or(WATCHLIST[0])
    }

    pub fn binance_selected_interval(&self) -> &'static str {
        INTERVALS
            .get(self.binance_interval_selected)
            .copied()
            .unwrap_or(INTERVALS[0])
    }

    pub fn home_menu(&self) -> HomeMenu {
        HomeMenu::ALL[self.home_selected]
    }

    pub fn work_selected_index(&self) -> usize {
        self.work_selected
    }

    pub fn secret_selected_index(&self) -> usize {
        self.secret_selected
    }

    pub fn focused_target(&self) -> Option<&BudgetTarget> {
        match self.panel_focus {
            PanelFocus::TargetA => self.visible_targets().first(),
            PanelFocus::TargetB => self.visible_targets().get(1),
            _ => None,
        }
    }

    pub fn pending_unlock_note_title(&self) -> Option<&str> {
        self.pending_unlock_note_id.and_then(|note_id| {
            self.secret_notes
                .iter()
                .find(|note| note.id == note_id)
                .map(|note| note.title.as_str())
        })
    }
}
