use std::fmt;

use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::state::LanguagePreset;

pub const CATEGORY_OPTIONS: [ExpenseCategory; 3] = [
    ExpenseCategory::Primer,
    ExpenseCategory::Sekunder,
    ExpenseCategory::Tersier,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExpenseCategory {
    #[serde(alias = "premier")]
    Primer,
    Sekunder,
    Tersier,
}

impl ExpenseCategory {
    pub fn as_str(self) -> &'static str {
        self.label(LanguagePreset::English)
    }

    pub fn label(self, language: LanguagePreset) -> &'static str {
        match self {
            Self::Primer => {
                if language.is_english() {
                    "Primary"
                } else {
                    "Primer"
                }
            }
            Self::Sekunder => {
                if language.is_english() {
                    "Secondary"
                } else {
                    "Sekunder"
                }
            }
            Self::Tersier => {
                if language.is_english() {
                    "Tertiary"
                } else {
                    "Tersier"
                }
            }
        }
    }

    pub fn from_stored(value: &str) -> Self {
        match value.trim().to_lowercase().as_str() {
            "premier" | "primer" => Self::Primer,
            "sekunder" => Self::Sekunder,
            "tersier" => Self::Tersier,
            _ => Self::Sekunder,
        }
    }
}

impl fmt::Display for ExpenseCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: u64,
    pub date: String,
    pub category: ExpenseCategory,
    pub description: String,
    pub amount: f64,
}

impl Expense {
    pub fn matches_filter(&self, query: &str) -> bool {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return true;
        }

        self.date.to_lowercase().contains(&query)
            || self.category.as_str().to_lowercase().contains(&query)
            || self.description.to_lowercase().contains(&query)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavingEntry {
    pub id: u64,
    pub date: String,
    pub description: String,
    pub amount: f64,
}

impl SavingEntry {
    pub fn matches_filter(&self, query: &str) -> bool {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return true;
        }

        self.date.to_lowercase().contains(&query)
            || self.description.to_lowercase().contains(&query)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningEntry {
    pub id: u64,
    pub date: String,
    pub description: String,
    pub amount: f64,
}

impl EarningEntry {
    pub fn matches_filter(&self, query: &str) -> bool {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return true;
        }

        self.date.to_lowercase().contains(&query)
            || self.description.to_lowercase().contains(&query)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretNote {
    pub id: u64,
    pub title: String,
    pub encrypted_content: String,
    pub passkey_hash: String,
    pub salt: String,
    pub created_at: String,
    #[serde(default)]
    pub last_unlocked_at: Option<String>,
}

impl SecretNote {
    pub fn matches_filter(&self, query: &str, unlocked_content: Option<&str>) -> bool {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return true;
        }

        self.title.to_lowercase().contains(&query)
            || self.created_at.to_lowercase().contains(&query)
            || unlocked_content.is_some_and(|content| content.to_lowercase().contains(&query))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTask {
    pub id: u64,
    pub description: String,
    pub deadline: String,
    #[serde(default = "default_deadline_time")]
    pub deadline_time: String,
    #[serde(default)]
    pub completed: bool,
    #[serde(default = "now_timestamp_string")]
    pub created_at: String,
    #[serde(default)]
    pub completed_at: Option<String>,
}

impl WorkTask {
    pub fn matches_filter(&self, query: &str) -> bool {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return true;
        }

        self.description.to_lowercase().contains(&query)
            || self.deadline.to_lowercase().contains(&query)
            || self.deadline_time.to_lowercase().contains(&query)
            || self
                .status_label(LanguagePreset::English)
                .to_lowercase()
                .contains(&query)
            || self
                .status_label(LanguagePreset::Indonesian)
                .to_lowercase()
                .contains(&query)
    }

    pub fn deadline_datetime(&self) -> Option<NaiveDateTime> {
        let date = NaiveDate::parse_from_str(&self.deadline, "%Y-%m-%d").ok()?;
        let time = NaiveTime::parse_from_str(&self.deadline_time, "%H:%M").ok()?;
        Some(date.and_time(time))
    }

    pub fn hours_until_deadline(&self) -> Option<i64> {
        let deadline = self.deadline_datetime()?;
        Some((deadline - Local::now().naive_local()).num_hours())
    }

    pub fn urgency(&self) -> WorkUrgency {
        if self.completed {
            return WorkUrgency::Done;
        }

        let Some(hours) = self.hours_until_deadline() else {
            return WorkUrgency::Unknown;
        };
        let today = Local::now().date_naive();
        let Some(deadline) = self.deadline_datetime() else {
            return WorkUrgency::Unknown;
        };

        if hours < 0 {
            WorkUrgency::Red
        } else if deadline.date() == today {
            WorkUrgency::Red
        } else if hours < 24 {
            WorkUrgency::Yellow
        } else {
            WorkUrgency::Green
        }
    }

    pub fn status_label(&self, language: LanguagePreset) -> &'static str {
        match self.urgency() {
            WorkUrgency::Done => {
                if language.is_english() {
                    "Done"
                } else {
                    "Selesai"
                }
            }
            WorkUrgency::Red => {
                if self.hours_until_deadline().is_some_and(|hours| hours < 0) {
                    if language.is_english() {
                        "Overdue"
                    } else {
                        "Lewat Deadline"
                    }
                } else if language.is_english() {
                    "Due Soon"
                } else {
                    "Hampir Deadline"
                }
            }
            WorkUrgency::Yellow => "H-1",
            WorkUrgency::Green => {
                if language.is_english() {
                    "On Track"
                } else {
                    "Aman"
                }
            }
            WorkUrgency::Unknown => {
                if language.is_english() {
                    "Unknown"
                } else {
                    "Tidak Valid"
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkUrgency {
    Red,
    Yellow,
    Green,
    Done,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackerMode {
    Expense,
    Saving,
    Earning,
}

impl TrackerMode {
    pub fn label(self, language: LanguagePreset) -> &'static str {
        match self {
            Self::Expense => {
                if language.is_english() {
                    "Expense"
                } else {
                    "Pengeluaran"
                }
            }
            Self::Saving => {
                if language.is_english() {
                    "Saving"
                } else {
                    "Nabung"
                }
            }
            Self::Earning => {
                if language.is_english() {
                    "Earning"
                } else {
                    "Pemasukan"
                }
            }
        }
    }
}

fn default_deadline_time() -> String {
    "17:00".to_string()
}

fn now_timestamp_string() -> String {
    Local::now().format("%Y-%m-%d %H:%M").to_string()
}
