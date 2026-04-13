mod accessors;
mod actions;
mod binance_actions;
mod binance_mutations;
mod messages;
mod metrics;
mod mutations;
mod navigation_actions;
mod reset_actions;
mod reset_mutations;
mod secret_actions;
mod secret_metrics;
mod secret_mutations;
mod target_actions;
mod work_actions;
mod work_metrics;
mod work_mutations;

use std::{collections::BTreeMap, path::PathBuf, thread::JoinHandle};

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::{
    binance::BinanceDashboard,
    models::{EarningEntry, Expense, SavingEntry, SecretNote, TrackerMode, WorkTask},
    state::{
        AppScreen, BalanceForm, BalanceState, BudgetTargetState, ConfirmChoice, EarningForm,
        ExpenseForm, LanguagePreset, Mode, PanelFocus, ResetForm, SavingForm, SecretNoteForm,
        TargetForm, ThemePreset, UnlockNoteForm, WorkForm,
    },
    storage,
};

pub(crate) const ANIMATION_FRAMES: u16 = 12;
type BinanceFetchJoin = JoinHandle<std::result::Result<BinanceDashboard, String>>;

#[derive(Debug)]
pub struct App {
    pub(crate) expenses: Vec<Expense>,
    pub(crate) savings: Vec<SavingEntry>,
    pub(crate) earnings: Vec<EarningEntry>,
    pub(crate) work_tasks: Vec<WorkTask>,
    pub(crate) secret_notes: Vec<SecretNote>,
    pub(crate) expense_selected: usize,
    pub(crate) saving_selected: usize,
    pub(crate) earning_selected: usize,
    pub(crate) work_selected: usize,
    pub(crate) secret_selected: usize,
    pub(crate) filter: String,
    pub(crate) tracker_mode: TrackerMode,
    pub(crate) screen: AppScreen,
    pub(crate) mode: Mode,
    pub(crate) expense_form: ExpenseForm,
    pub(crate) saving_form: SavingForm,
    pub(crate) earning_form: EarningForm,
    pub(crate) work_form: WorkForm,
    pub(crate) secret_note_form: SecretNoteForm,
    pub(crate) unlock_note_form: UnlockNoteForm,
    pub(crate) balance_form: BalanceForm,
    pub(crate) balance: BalanceState,
    pub(crate) target_form: TargetForm,
    pub(crate) reset_form: ResetForm,
    pub(crate) targets: BudgetTargetState,
    pub(crate) status: String,
    pub(crate) data_path: PathBuf,
    pub(crate) next_expense_id: u64,
    pub(crate) next_saving_id: u64,
    pub(crate) next_earning_id: u64,
    pub(crate) next_work_id: u64,
    pub(crate) next_secret_note_id: u64,
    pub(crate) next_target_id: u64,
    pub(crate) animation_tick: u64,
    pub(crate) animation_frames_left: u16,
    pub(crate) celebration_frames_left: u16,
    pub(crate) celebration_target_title: String,
    pub(crate) celebration_success: bool,
    pub(crate) work_delete_frames_left: u16,
    pub(crate) work_delete_task_title: String,
    pub(crate) work_delete_was_completed: bool,
    pub(crate) theme_preset: ThemePreset,
    pub(crate) theme_selected: usize,
    pub(crate) language_preset: LanguagePreset,
    pub(crate) language_selected: usize,
    pub(crate) home_selected: usize,
    pub(crate) binance_watchlist_selected: usize,
    pub(crate) binance_symbol_selected: usize,
    pub(crate) binance_balance_selected: usize,
    pub(crate) binance_interval_selected: usize,
    pub(crate) binance_dashboard: BinanceDashboard,
    pub(crate) binance_loading: bool,
    pub(crate) binance_refresh_queued: bool,
    pub(crate) binance_fetch_handle: Option<BinanceFetchJoin>,
    pub(crate) target_page: usize,
    pub(crate) panel_focus: PanelFocus,
    pub(crate) pending_delete_target_id: Option<u64>,
    pub(crate) pending_unlock_note_id: Option<u64>,
    pub(crate) editing_secret_note_id: Option<u64>,
    pub(crate) unlocked_notes: BTreeMap<u64, String>,
    pub(crate) confirm_choice: ConfirmChoice,
}

impl App {
    pub fn load(data_path: PathBuf) -> Result<Self> {
        let (
            expenses,
            savings,
            earnings,
            work_tasks,
            secret_notes,
            balance,
            targets,
            theme_preset,
            language_preset,
        ) = storage::load_data(&data_path)?;
        let next_expense_id = expenses.iter().map(|item| item.id).max().unwrap_or(0) + 1;
        let next_saving_id = savings.iter().map(|item| item.id).max().unwrap_or(0) + 1;
        let next_earning_id = earnings.iter().map(|item| item.id).max().unwrap_or(0) + 1;
        let next_work_id = work_tasks.iter().map(|item| item.id).max().unwrap_or(0) + 1;
        let next_secret_note_id = secret_notes.iter().map(|item| item.id).max().unwrap_or(0) + 1;
        let next_target_id = targets.items.iter().map(|item| item.id).max().unwrap_or(0) + 1;

        let mut app = Self {
            expenses,
            savings,
            earnings,
            work_tasks,
            secret_notes,
            expense_selected: 0,
            saving_selected: 0,
            earning_selected: 0,
            work_selected: 0,
            secret_selected: 0,
            filter: String::new(),
            tracker_mode: TrackerMode::Expense,
            screen: AppScreen::Home,
            mode: Mode::Normal,
            expense_form: ExpenseForm::default(),
            saving_form: SavingForm::default(),
            earning_form: EarningForm::default(),
            work_form: WorkForm::default(),
            secret_note_form: SecretNoteForm::default(),
            unlock_note_form: UnlockNoteForm::default(),
            balance_form: BalanceForm::default(),
            target_form: TargetForm::default(),
            reset_form: ResetForm::default(),
            balance,
            targets,
            status: format!("Data file: {}", data_path.display()),
            data_path,
            next_expense_id,
            next_saving_id,
            next_earning_id,
            next_work_id,
            next_secret_note_id,
            next_target_id,
            animation_tick: 0,
            animation_frames_left: ANIMATION_FRAMES,
            celebration_frames_left: 0,
            celebration_target_title: String::new(),
            celebration_success: false,
            work_delete_frames_left: 0,
            work_delete_task_title: String::new(),
            work_delete_was_completed: false,
            theme_preset,
            theme_selected: ThemePreset::ALL
                .iter()
                .position(|preset| *preset == theme_preset)
                .unwrap_or(0),
            language_preset,
            language_selected: LanguagePreset::ALL
                .iter()
                .position(|preset| *preset == language_preset)
                .unwrap_or(0),
            home_selected: 0,
            binance_watchlist_selected: 0,
            binance_symbol_selected: 0,
            binance_balance_selected: 0,
            binance_interval_selected: 1,
            binance_dashboard: BinanceDashboard::default(),
            binance_loading: false,
            binance_refresh_queued: false,
            binance_fetch_handle: None,
            target_page: 0,
            panel_focus: PanelFocus::Table,
            pending_delete_target_id: None,
            pending_unlock_note_id: None,
            editing_secret_note_id: None,
            unlocked_notes: BTreeMap::new(),
            confirm_choice: ConfirmChoice::No,
        };
        app.clamp_selection();
        Ok(app)
    }

    pub fn tick(&mut self) {
        self.poll_binance_refresh();
        if self.animation_frames_left > 0 {
            self.animation_tick = self.animation_tick.saturating_add(1);
            self.animation_frames_left -= 1;
        }
        if self.celebration_frames_left > 0 {
            self.celebration_frames_left -= 1;
        }
        if self.work_delete_frames_left > 0 {
            self.work_delete_frames_left -= 1;
        }
    }

    fn poll_binance_refresh(&mut self) {
        let finished = self
            .binance_fetch_handle
            .as_ref()
            .is_some_and(std::thread::JoinHandle::is_finished);
        if !finished {
            return;
        }

        let Some(handle) = self.binance_fetch_handle.take() else {
            return;
        };
        self.binance_loading = false;

        match handle.join() {
            Ok(Ok(dashboard)) => {
                self.binance_watchlist_selected =
                    crate::binance::watchlist_index(&dashboard.symbol);
                self.binance_symbol_selected = self.binance_watchlist_selected;
                self.binance_interval_selected =
                    crate::binance::interval_index(&dashboard.interval);
                self.binance_dashboard = dashboard;
                self.clamp_selection();
                self.status = if self.is_english() {
                    format!(
                        "Binance sync complete for {} / {}.",
                        self.binance_dashboard.symbol, self.binance_dashboard.interval
                    )
                } else {
                    format!(
                        "Sync Binance selesai untuk {} / {}.",
                        self.binance_dashboard.symbol, self.binance_dashboard.interval
                    )
                };
            }
            Ok(Err(message)) => {
                self.status = message;
            }
            Err(_) => {
                self.status = if self.is_english() {
                    "Binance worker stopped unexpectedly.".into()
                } else {
                    "Worker Binance berhenti tidak terduga.".into()
                };
            }
        }

        if self.binance_refresh_queued {
            self.binance_refresh_queued = false;
            self.spawn_binance_refresh();
        }
    }

    pub fn set_error(&mut self, message: String) {
        self.status = message;
        self.mode = Mode::Normal;
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Result<bool> {
        if self.screen == AppScreen::Home {
            return match self.mode {
                Mode::ThemeSelect => self.handle_theme_selector_key(key),
                Mode::LanguageSelect => self.handle_language_selector_key(key),
                _ => self.handle_home_key(key),
            };
        }

        if self.screen == AppScreen::WorkTracker {
            return match self.mode {
                Mode::Normal => self.handle_work_normal_key(key),
                Mode::Filter => self.handle_filter_key(key),
                Mode::AddWork => self.handle_add_work_key(key),
                Mode::ResetAll => self.handle_reset_all_key(key),
                Mode::ThemeSelect => self.handle_theme_selector_key(key),
                Mode::LanguageSelect => self.handle_language_selector_key(key),
                _ => Ok(false),
            };
        }

        if self.screen == AppScreen::SecretNotes {
            return match self.mode {
                Mode::Normal => self.handle_secret_normal_key(key),
                Mode::Filter => self.handle_filter_key(key),
                Mode::AddSecretNote => self.handle_add_secret_note_key(key),
                Mode::EditSecretNote => self.handle_edit_secret_note_key(key),
                Mode::UnlockSecretNote => self.handle_unlock_secret_note_key(key),
                Mode::ResetAll => self.handle_reset_all_key(key),
                Mode::ThemeSelect => self.handle_theme_selector_key(key),
                Mode::LanguageSelect => self.handle_language_selector_key(key),
                _ => Ok(false),
            };
        }

        if self.screen == AppScreen::BinanceTracker {
            return match self.mode {
                Mode::Normal => self.handle_binance_normal_key(key),
                Mode::ThemeSelect => self.handle_theme_selector_key(key),
                Mode::LanguageSelect => self.handle_language_selector_key(key),
                _ => Ok(false),
            };
        }

        match self.mode {
            Mode::Normal => self.handle_normal_key(key),
            Mode::Filter => self.handle_filter_key(key),
            Mode::AddExpense => self.handle_add_expense_key(key),
            Mode::AddSaving => self.handle_add_saving_key(key),
            Mode::AddEarning => self.handle_add_earning_key(key),
            Mode::AddWork => Ok(false),
            Mode::AddSecretNote => Ok(false),
            Mode::EditSecretNote => Ok(false),
            Mode::UnlockSecretNote => Ok(false),
            Mode::AddBalance => self.handle_add_balance_key(key),
            Mode::AddTarget => self.handle_add_target_key(key),
            Mode::ResetAll => self.handle_reset_all_key(key),
            Mode::ConfirmTargetDelete => self.handle_confirm_target_delete_key(key),
            Mode::ThemeSelect => self.handle_theme_selector_key(key),
            Mode::LanguageSelect => self.handle_language_selector_key(key),
        }
    }
}
