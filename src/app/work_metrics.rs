use std::collections::BTreeMap;

use chrono::NaiveDateTime;

use super::App;
use crate::{
    models::{WorkTask, WorkUrgency},
    state::AppScreen,
};

impl App {
    pub fn filtered_work_indices(&self) -> Vec<usize> {
        let mut indices = self
            .work_tasks
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| item.matches_filter(&self.filter).then_some(idx))
            .collect::<Vec<_>>();

        indices.sort_by(|a, b| self.compare_work_tasks(*a, *b));
        indices
    }

    pub fn work_at(&self, idx: usize) -> Option<&WorkTask> {
        self.work_tasks.get(idx)
    }

    pub fn selected_work(&self) -> Option<&WorkTask> {
        self.filtered_work_indices()
            .get(self.work_selected)
            .and_then(|idx| self.work_tasks.get(*idx))
    }

    pub fn work_pending_count(&self) -> usize {
        self.work_tasks
            .iter()
            .filter(|item| !item.completed)
            .count()
    }

    pub fn work_completed_count(&self) -> usize {
        self.work_tasks.iter().filter(|item| item.completed).count()
    }

    pub fn work_red_zone_count(&self) -> usize {
        self.work_tasks
            .iter()
            .filter(|item| matches!(item.urgency(), WorkUrgency::Red))
            .count()
    }

    pub fn work_h1_count(&self) -> usize {
        self.work_tasks
            .iter()
            .filter(|item| matches!(item.urgency(), WorkUrgency::Yellow))
            .count()
    }

    pub fn work_completion_rate(&self) -> f64 {
        if self.work_tasks.is_empty() {
            0.0
        } else {
            (self.work_completed_count() as f64 / self.work_tasks.len() as f64) * 100.0
        }
    }

    pub fn work_urgency_buckets(&self, english: bool) -> Vec<(&'static str, u64)> {
        let mut red = 0;
        let mut yellow = 0;
        let mut green = 0;

        for item in self.work_tasks.iter().filter(|item| !item.completed) {
            match item.urgency() {
                WorkUrgency::Red => red += 1,
                WorkUrgency::Yellow => yellow += 1,
                WorkUrgency::Green => green += 1,
                WorkUrgency::Done | WorkUrgency::Unknown => {}
            }
        }

        vec![
            (if english { "Red" } else { "Merah" }, red),
            (if english { "Yellow" } else { "Kuning" }, yellow),
            (if english { "Green" } else { "Hijau" }, green),
        ]
    }

    pub fn work_status_buckets(&self, english: bool) -> Vec<(&'static str, u64)> {
        vec![
            (
                if english { "Pending" } else { "Belum" },
                self.work_pending_count() as u64,
            ),
            (
                if english { "Done" } else { "Selesai" },
                self.work_completed_count() as u64,
            ),
        ]
    }

    pub fn work_completed_by_day(&self) -> Vec<(String, u64)> {
        let mut totals: BTreeMap<String, u64> = BTreeMap::new();
        for item in self.work_tasks.iter().filter(|item| item.completed) {
            if let Some(date) = &item.completed_at {
                *totals.entry(date.clone()).or_default() += 1;
            }
        }

        let mut items = totals.into_iter().collect::<Vec<_>>();
        items.sort_by(|a, b| a.0.cmp(&b.0));
        if items.len() > 5 {
            items = items[items.len() - 5..].to_vec();
        }
        items
    }

    pub fn work_created_by_day(&self) -> Vec<(String, u64)> {
        let mut totals: BTreeMap<String, u64> = BTreeMap::new();
        for item in &self.work_tasks {
            *totals.entry(item.created_at.clone()).or_default() += 1;
        }

        let mut items = totals.into_iter().collect::<Vec<_>>();
        items.sort_by(|a, b| a.0.cmp(&b.0));
        if items.len() > 5 {
            items = items[items.len() - 5..].to_vec();
        }
        items
    }

    pub fn top_work_focus(&self) -> String {
        if let Some(task) = self
            .work_tasks
            .iter()
            .filter(|item| !item.completed)
            .min_by(|a, b| self.compare_work_dates(a, b))
        {
            task.status_label(self.language_preset).to_string()
        } else if self.is_english() {
            "Stable".into()
        } else {
            "Stabil".into()
        }
    }

    pub(crate) fn current_len(&self) -> usize {
        match self.screen {
            AppScreen::WorkTracker => self.filtered_work_indices().len(),
            AppScreen::ExpenseTracker => match self.tracker_mode {
                crate::models::TrackerMode::Expense => self.filtered_expense_indices().len(),
                crate::models::TrackerMode::Saving => self.filtered_saving_indices().len(),
                crate::models::TrackerMode::Earning => self.filtered_earning_indices().len(),
            },
            AppScreen::SecretNotes => self.filtered_secret_note_indices().len(),
            AppScreen::BinanceTracker => match self.panel_focus {
                crate::state::PanelFocus::Table => self.binance_watchlist().len(),
                crate::state::PanelFocus::TargetA => self.binance_dashboard.balances.len(),
                _ => 0,
            },
            AppScreen::Home => 0,
        }
    }

    pub(crate) fn selected(&self) -> usize {
        match self.screen {
            AppScreen::WorkTracker => self.work_selected,
            AppScreen::ExpenseTracker => match self.tracker_mode {
                crate::models::TrackerMode::Expense => self.expense_selected,
                crate::models::TrackerMode::Saving => self.saving_selected,
                crate::models::TrackerMode::Earning => self.earning_selected,
            },
            AppScreen::SecretNotes => self.secret_selected,
            AppScreen::BinanceTracker => match self.panel_focus {
                crate::state::PanelFocus::Table => self.binance_watchlist_selected,
                crate::state::PanelFocus::TargetA => self.binance_balance_selected,
                _ => 0,
            },
            AppScreen::Home => 0,
        }
    }

    pub(crate) fn set_selected(&mut self, value: usize) {
        match self.screen {
            AppScreen::WorkTracker => self.work_selected = value,
            AppScreen::ExpenseTracker => match self.tracker_mode {
                crate::models::TrackerMode::Expense => self.expense_selected = value,
                crate::models::TrackerMode::Saving => self.saving_selected = value,
                crate::models::TrackerMode::Earning => self.earning_selected = value,
            },
            AppScreen::SecretNotes => self.secret_selected = value,
            AppScreen::BinanceTracker => match self.panel_focus {
                crate::state::PanelFocus::Table => self.binance_watchlist_selected = value,
                crate::state::PanelFocus::TargetA => self.binance_balance_selected = value,
                _ => {}
            },
            AppScreen::Home => {}
        }
    }

    pub(crate) fn clamp_selection(&mut self) {
        let len = self.current_len();
        let clamped = if len == 0 {
            0
        } else {
            self.selected().min(len - 1)
        };
        self.set_selected(clamped);
    }

    fn compare_work_tasks(&self, a: usize, b: usize) -> std::cmp::Ordering {
        let left = &self.work_tasks[a];
        let right = &self.work_tasks[b];
        left.completed
            .cmp(&right.completed)
            .then_with(|| self.compare_work_dates(left, right))
            .then_with(|| left.description.cmp(&right.description))
    }

    fn compare_work_dates(&self, a: &WorkTask, b: &WorkTask) -> std::cmp::Ordering {
        parse_datetime(a)
            .cmp(&parse_datetime(b))
            .then_with(|| a.deadline.cmp(&b.deadline))
            .then_with(|| a.deadline_time.cmp(&b.deadline_time))
    }
}

fn parse_datetime(task: &WorkTask) -> Option<NaiveDateTime> {
    task.deadline_datetime()
}
