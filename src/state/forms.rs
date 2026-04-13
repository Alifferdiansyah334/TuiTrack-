use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::models::{CATEGORY_OPTIONS, ExpenseCategory};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BalanceState {
    pub enabled: bool,
    pub amount: f64,
}

impl Default for BalanceState {
    fn default() -> Self {
        Self {
            enabled: false,
            amount: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BalanceFormField {
    Enabled,
    Amount,
}

impl BalanceFormField {
    pub fn next(self) -> Self {
        match self {
            Self::Enabled => Self::Amount,
            Self::Amount => Self::Amount,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Enabled => Self::Enabled,
            Self::Amount => Self::Enabled,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BalanceForm {
    pub enabled: bool,
    pub amount: String,
    pub focus: BalanceFormField,
}

impl Default for BalanceForm {
    fn default() -> Self {
        Self {
            enabled: false,
            amount: String::new(),
            focus: BalanceFormField::Enabled,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpenseFormField {
    Date,
    Category,
    Description,
    Amount,
}

impl ExpenseFormField {
    pub fn next(self) -> Self {
        match self {
            Self::Date => Self::Category,
            Self::Category => Self::Description,
            Self::Description => Self::Amount,
            Self::Amount => Self::Amount,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Date => Self::Date,
            Self::Category => Self::Date,
            Self::Description => Self::Category,
            Self::Amount => Self::Description,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExpenseForm {
    pub date: String,
    pub category_index: usize,
    pub description: String,
    pub amount: String,
    pub focus: ExpenseFormField,
}

impl Default for ExpenseForm {
    fn default() -> Self {
        Self {
            date: today_string(),
            category_index: 0,
            description: String::new(),
            amount: String::new(),
            focus: ExpenseFormField::Date,
        }
    }
}

impl ExpenseForm {
    pub fn category(&self) -> ExpenseCategory {
        CATEGORY_OPTIONS[self.category_index]
    }

    pub fn next_category(&mut self) {
        self.category_index = (self.category_index + 1) % CATEGORY_OPTIONS.len();
    }

    pub fn prev_category(&mut self) {
        if self.category_index == 0 {
            self.category_index = CATEGORY_OPTIONS.len() - 1;
        } else {
            self.category_index -= 1;
        }
    }

    pub(crate) fn current_value_mut(&mut self) -> Option<&mut String> {
        match self.focus {
            ExpenseFormField::Date => Some(&mut self.date),
            ExpenseFormField::Category => None,
            ExpenseFormField::Description => Some(&mut self.description),
            ExpenseFormField::Amount => Some(&mut self.amount),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavingFormField {
    Date,
    Description,
    Amount,
}

impl SavingFormField {
    pub fn next(self) -> Self {
        match self {
            Self::Date => Self::Description,
            Self::Description => Self::Amount,
            Self::Amount => Self::Amount,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Date => Self::Date,
            Self::Description => Self::Date,
            Self::Amount => Self::Description,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SavingForm {
    pub date: String,
    pub description: String,
    pub amount: String,
    pub focus: SavingFormField,
}

impl Default for SavingForm {
    fn default() -> Self {
        Self {
            date: today_string(),
            description: String::new(),
            amount: String::new(),
            focus: SavingFormField::Date,
        }
    }
}

impl SavingForm {
    pub(crate) fn current_value_mut(&mut self) -> &mut String {
        match self.focus {
            SavingFormField::Date => &mut self.date,
            SavingFormField::Description => &mut self.description,
            SavingFormField::Amount => &mut self.amount,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EarningFormField {
    Date,
    Description,
    Amount,
}

impl EarningFormField {
    pub fn next(self) -> Self {
        match self {
            Self::Date => Self::Description,
            Self::Description => Self::Amount,
            Self::Amount => Self::Amount,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Date => Self::Date,
            Self::Description => Self::Date,
            Self::Amount => Self::Description,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EarningForm {
    pub date: String,
    pub description: String,
    pub amount: String,
    pub focus: EarningFormField,
}

impl Default for EarningForm {
    fn default() -> Self {
        Self {
            date: today_string(),
            description: String::new(),
            amount: String::new(),
            focus: EarningFormField::Date,
        }
    }
}

impl EarningForm {
    pub(crate) fn current_value_mut(&mut self) -> &mut String {
        match self.focus {
            EarningFormField::Date => &mut self.date,
            EarningFormField::Description => &mut self.description,
            EarningFormField::Amount => &mut self.amount,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResetForm {
    pub confirmation: String,
}

impl Default for ResetForm {
    fn default() -> Self {
        Self {
            confirmation: String::new(),
        }
    }
}

fn today_string() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}
