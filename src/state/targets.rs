use serde::{Deserialize, Serialize};

use super::LanguagePreset;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetTarget {
    pub id: u64,
    pub title: String,
    pub mode: TargetMode,
    pub amount: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BudgetTargetState {
    #[serde(default)]
    pub items: Vec<BudgetTarget>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetMode {
    Saving,
    TotalBalance,
}

impl TargetMode {
    pub fn label(self, language: LanguagePreset) -> &'static str {
        match self {
            Self::Saving => {
                if language.is_english() {
                    "By Saving"
                } else {
                    "By Tabungan"
                }
            }
            Self::TotalBalance => {
                if language.is_english() {
                    "By Total"
                } else {
                    "By Total"
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetFormField {
    Title,
    Kind,
    Amount,
}

impl TargetFormField {
    pub fn next(self) -> Self {
        match self {
            Self::Title => Self::Kind,
            Self::Kind => Self::Amount,
            Self::Amount => Self::Amount,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Title => Self::Title,
            Self::Kind => Self::Title,
            Self::Amount => Self::Kind,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TargetForm {
    pub title: String,
    pub mode: TargetMode,
    pub amount: String,
    pub focus: TargetFormField,
}

impl Default for TargetForm {
    fn default() -> Self {
        Self {
            title: String::new(),
            mode: TargetMode::Saving,
            amount: String::new(),
            focus: TargetFormField::Title,
        }
    }
}
