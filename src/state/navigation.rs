use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Home,
    ExpenseTracker,
    WorkTracker,
    SecretNotes,
    BinanceTracker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomeMenu {
    ExpenseTracker,
    WorkTracker,
    SecretNotes,
    BinanceTracker,
}

impl HomeMenu {
    pub const ALL: [Self; 4] = [
        Self::ExpenseTracker,
        Self::WorkTracker,
        Self::SecretNotes,
        Self::BinanceTracker,
    ];

    pub fn label(self, language: LanguagePreset) -> &'static str {
        match self {
            Self::ExpenseTracker => {
                if language.is_english() {
                    "Expense Tracker"
                } else {
                    "Pelacak Expense"
                }
            }
            Self::WorkTracker => {
                if language.is_english() {
                    "Work Tracker"
                } else {
                    "Pelacak Kerja"
                }
            }
            Self::SecretNotes => {
                if language.is_english() {
                    "Secret Notes"
                } else {
                    "Catatan Rahasia"
                }
            }
            Self::BinanceTracker => {
                if language.is_english() {
                    "Binance Tracker"
                } else {
                    "Pelacak Binance"
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemePreset {
    Forest,
    Amber,
    Ocean,
    Mono,
}

impl ThemePreset {
    pub const ALL: [Self; 4] = [Self::Forest, Self::Amber, Self::Ocean, Self::Mono];

    pub fn label(self) -> &'static str {
        match self {
            Self::Forest => "Forest",
            Self::Amber => "Amber",
            Self::Ocean => "Ocean",
            Self::Mono => "Mono",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LanguagePreset {
    Indonesian,
    English,
}

impl LanguagePreset {
    pub const ALL: [Self; 2] = [Self::Indonesian, Self::English];

    pub fn label(self) -> &'static str {
        match self {
            Self::Indonesian => "Indonesia",
            Self::English => "English",
        }
    }

    pub fn is_english(self) -> bool {
        matches!(self, Self::English)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Filter,
    AddExpense,
    AddSaving,
    AddEarning,
    AddWork,
    AddSecretNote,
    EditSecretNote,
    UnlockSecretNote,
    AddBalance,
    AddTarget,
    ResetAll,
    ConfirmTargetDelete,
    ThemeSelect,
    LanguageSelect,
}

impl Mode {
    pub fn label(self, language: LanguagePreset) -> &'static str {
        match self {
            Self::Normal => {
                if language.is_english() {
                    "Normal"
                } else {
                    "Normal"
                }
            }
            Self::Filter => {
                if language.is_english() {
                    "Filter"
                } else {
                    "Filter"
                }
            }
            Self::AddExpense => {
                if language.is_english() {
                    "Add Expense"
                } else {
                    "Tambah Expense"
                }
            }
            Self::AddSaving => {
                if language.is_english() {
                    "Add Saving"
                } else {
                    "Tambah Nabung"
                }
            }
            Self::AddEarning => {
                if language.is_english() {
                    "Add Earning"
                } else {
                    "Tambah Pemasukan"
                }
            }
            Self::AddWork => {
                if language.is_english() {
                    "Add Task"
                } else {
                    "Tambah Tugas"
                }
            }
            Self::AddSecretNote => {
                if language.is_english() {
                    "Add Secret Note"
                } else {
                    "Tambah Catatan Rahasia"
                }
            }
            Self::EditSecretNote => {
                if language.is_english() {
                    "Edit Secret Note"
                } else {
                    "Edit Catatan Rahasia"
                }
            }
            Self::UnlockSecretNote => {
                if language.is_english() {
                    "Unlock Secret Note"
                } else {
                    "Buka Catatan Rahasia"
                }
            }
            Self::AddBalance => {
                if language.is_english() {
                    "Set Balance"
                } else {
                    "Atur Balance"
                }
            }
            Self::AddTarget => {
                if language.is_english() {
                    "Set Target"
                } else {
                    "Atur Target"
                }
            }
            Self::ResetAll => {
                if language.is_english() {
                    "Reset Data"
                } else {
                    "Reset Data"
                }
            }
            Self::ConfirmTargetDelete => {
                if language.is_english() {
                    "Delete Target"
                } else {
                    "Hapus Target"
                }
            }
            Self::ThemeSelect => {
                if language.is_english() {
                    "Theme Selector"
                } else {
                    "Pemilih Tema"
                }
            }
            Self::LanguageSelect => {
                if language.is_english() {
                    "Language Selector"
                } else {
                    "Pemilih Bahasa"
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelFocus {
    Table,
    ExpenseChart,
    SavingChart,
    TargetA,
    TargetB,
}

impl PanelFocus {
    pub const ORDER: [Self; 5] = [
        Self::Table,
        Self::ExpenseChart,
        Self::SavingChart,
        Self::TargetA,
        Self::TargetB,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmChoice {
    Yes,
    No,
}
