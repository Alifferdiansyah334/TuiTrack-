use std::collections::BTreeMap;

use chrono::Local;

use super::App;
use crate::models::{CATEGORY_OPTIONS, EarningEntry, Expense, ExpenseCategory, SavingEntry};
use crate::state::{BudgetTarget, TargetMode};

impl App {
    pub fn total_expense_amount(&self) -> f64 {
        self.expenses.iter().map(|item| item.amount).sum()
    }

    pub fn monthly_expense_amount(&self) -> f64 {
        let prefix = Local::now().format("%Y-%m").to_string();
        self.expenses
            .iter()
            .filter(|item| item.date.starts_with(&prefix))
            .map(|item| item.amount)
            .sum()
    }

    pub fn total_saving_amount(&self) -> f64 {
        self.savings.iter().map(|item| item.amount).sum()
    }

    pub fn total_earning_amount(&self) -> f64 {
        self.earnings.iter().map(|item| item.amount).sum()
    }

    pub fn monthly_saving_amount(&self) -> f64 {
        let prefix = Local::now().format("%Y-%m").to_string();
        self.savings
            .iter()
            .filter(|item| item.date.starts_with(&prefix))
            .map(|item| item.amount)
            .sum()
    }

    pub fn monthly_earning_amount(&self) -> f64 {
        let prefix = Local::now().format("%Y-%m").to_string();
        self.earnings
            .iter()
            .filter(|item| item.date.starts_with(&prefix))
            .map(|item| item.amount)
            .sum()
    }

    pub fn net_balance(&self) -> f64 {
        self.effective_base_balance() + self.total_saving_amount() + self.total_earning_amount()
            - self.total_expense_amount()
    }

    pub fn effective_base_balance(&self) -> f64 {
        if self.balance.enabled {
            self.balance.amount
        } else {
            0.0
        }
    }

    pub fn target_progress(&self, target: &BudgetTarget) -> (f64, f64, f64, f64) {
        let current = match target.mode {
            TargetMode::Saving => self.total_saving_amount(),
            TargetMode::TotalBalance => self.net_balance(),
        };
        let (achieved, remaining, percent) = progress_parts(current, target.amount);
        (current, achieved, remaining, percent)
    }

    pub fn visible_targets(&self) -> &[BudgetTarget] {
        let start = self.target_page.saturating_mul(2);
        let end = (start + 2).min(self.targets.items.len());
        &self.targets.items[start..end]
    }

    pub fn filtered_expense_indices(&self) -> Vec<usize> {
        self.expenses
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| item.matches_filter(&self.filter).then_some(idx))
            .collect()
    }

    pub fn filtered_saving_indices(&self) -> Vec<usize> {
        self.savings
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| item.matches_filter(&self.filter).then_some(idx))
            .collect()
    }

    pub fn filtered_earning_indices(&self) -> Vec<usize> {
        self.earnings
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| item.matches_filter(&self.filter).then_some(idx))
            .collect()
    }

    pub fn expense_at(&self, idx: usize) -> Option<&Expense> {
        self.expenses.get(idx)
    }

    pub fn saving_at(&self, idx: usize) -> Option<&SavingEntry> {
        self.savings.get(idx)
    }

    pub fn earning_at(&self, idx: usize) -> Option<&EarningEntry> {
        self.earnings.get(idx)
    }

    pub fn selected_expense(&self) -> Option<&Expense> {
        self.filtered_expense_indices()
            .get(self.expense_selected)
            .and_then(|idx| self.expenses.get(*idx))
    }

    pub fn selected_saving(&self) -> Option<&SavingEntry> {
        self.filtered_saving_indices()
            .get(self.saving_selected)
            .and_then(|idx| self.savings.get(*idx))
    }

    pub fn selected_earning(&self) -> Option<&EarningEntry> {
        self.filtered_earning_indices()
            .get(self.earning_selected)
            .and_then(|idx| self.earnings.get(*idx))
    }

    pub fn expense_category_totals(&self) -> Vec<(ExpenseCategory, f64)> {
        CATEGORY_OPTIONS
            .iter()
            .map(|category| {
                let total = self
                    .expenses
                    .iter()
                    .filter(|item| item.category == *category)
                    .map(|item| item.amount)
                    .sum();
                (*category, total)
            })
            .collect()
    }

    pub fn top_expense_category(&self) -> (ExpenseCategory, f64) {
        self.expense_category_totals()
            .into_iter()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .unwrap_or((ExpenseCategory::Primer, 0.0))
    }

    pub fn saving_totals_by_day(&self) -> Vec<(String, f64)> {
        let mut totals: BTreeMap<String, f64> = BTreeMap::new();
        for item in &self.savings {
            *totals.entry(item.date.clone()).or_default() += item.amount;
        }

        let mut entries = totals.into_iter().collect::<Vec<_>>();
        entries.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        entries
    }

    pub fn top_saving_day(&self) -> (String, f64) {
        self.saving_totals_by_day()
            .into_iter()
            .next()
            .unwrap_or_else(|| ("-".into(), 0.0))
    }

    pub fn earning_totals_by_day(&self) -> Vec<(String, f64)> {
        let mut totals: BTreeMap<String, f64> = BTreeMap::new();
        for item in &self.earnings {
            *totals.entry(item.date.clone()).or_default() += item.amount;
        }

        let mut entries = totals.into_iter().collect::<Vec<_>>();
        entries.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        entries
    }

    pub fn top_earning_day(&self) -> (String, f64) {
        self.earning_totals_by_day()
            .into_iter()
            .next()
            .unwrap_or_else(|| ("-".into(), 0.0))
    }

    pub fn animated_chart_value(&self, amount: f64, offset: usize) -> u64 {
        let base = chart_value(amount);
        if base == 0 {
            return 0;
        }

        let delay = (offset as u64) * 2;
        let effective_tick = self.animation_tick.saturating_sub(delay);
        let progress = (effective_tick as f64 / super::ANIMATION_FRAMES as f64).clamp(0.0, 1.0);
        let eased = 1.0 - (1.0 - progress) * (1.0 - progress);
        ((base as f64) * eased).round().max(1.0) as u64
    }

    pub(crate) fn move_selection(&mut self, delta: isize) {
        let len = self.current_len();
        if len == 0 {
            self.set_selected(0);
            return;
        }

        let selected = self.selected() as isize;
        let next = (selected + delta).clamp(0, len.saturating_sub(1) as isize) as usize;
        self.set_selected(next);
    }
}

fn chart_value(amount: f64) -> u64 {
    amount.round().max(0.0) as u64
}

fn progress_parts(current: f64, target: f64) -> (f64, f64, f64) {
    if target <= 0.0 {
        return (0.0, 0.0, 0.0);
    }

    let current_positive = current.max(0.0);
    let achieved = current_positive.min(target);
    let remaining = (target - achieved).max(0.0);
    let percent = (current_positive / target * 100.0).clamp(0.0, 100.0);
    (achieved, remaining, percent)
}
