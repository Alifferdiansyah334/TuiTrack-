use std::{fs, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::models::{EarningEntry, Expense, ExpenseCategory, SavingEntry, SecretNote, WorkTask};
use crate::state::{
    BalanceState, BudgetTarget, BudgetTargetState, LanguagePreset, TargetMode, ThemePreset,
};

#[derive(Debug, Serialize)]
struct AppDataFileRef<'a> {
    expenses: &'a [Expense],
    savings: &'a [SavingEntry],
    earnings: &'a [EarningEntry],
    work_tasks: &'a [WorkTask],
    secret_notes: &'a [SecretNote],
    balance: &'a BalanceState,
    targets: &'a BudgetTargetState,
    theme: ThemePreset,
    language: LanguagePreset,
}

#[derive(Debug, Deserialize)]
struct AppDataFile {
    expenses: Vec<Expense>,
    savings: Vec<SavingEntry>,
    #[serde(default)]
    earnings: Vec<EarningEntry>,
    #[serde(default)]
    work_tasks: Vec<WorkTask>,
    #[serde(default)]
    secret_notes: Vec<SecretNote>,
    #[serde(default)]
    balance: BalanceState,
    #[serde(default)]
    targets: serde_json::Value,
    #[serde(default = "default_theme")]
    theme: ThemePreset,
    #[serde(default = "default_language")]
    language: LanguagePreset,
}

#[derive(Debug, Deserialize)]
struct LegacyExpense {
    id: u64,
    date: String,
    category: String,
    description: String,
    amount: f64,
}

pub fn save_data(
    path: &Path,
    expenses: &[Expense],
    savings: &[SavingEntry],
    earnings: &[EarningEntry],
    work_tasks: &[WorkTask],
    secret_notes: &[SecretNote],
    balance: &BalanceState,
    targets: &BudgetTargetState,
    theme: ThemePreset,
    language: LanguagePreset,
) -> Result<()> {
    let content = serde_json::to_string_pretty(&AppDataFileRef {
        expenses,
        savings,
        earnings,
        work_tasks,
        secret_notes,
        balance,
        targets,
        theme,
        language,
    })
    .context("gagal serialize data")?;
    fs::write(path, content).with_context(|| format!("gagal menyimpan {}", path.display()))
}

pub fn load_data(
    path: &Path,
) -> Result<(
    Vec<Expense>,
    Vec<SavingEntry>,
    Vec<EarningEntry>,
    Vec<WorkTask>,
    Vec<SecretNote>,
    BalanceState,
    BudgetTargetState,
    ThemePreset,
    LanguagePreset,
)> {
    if !path.exists() {
        return Ok((
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            BalanceState::default(),
            BudgetTargetState::default(),
            default_theme(),
            default_language(),
        ));
    }

    let content =
        fs::read_to_string(path).with_context(|| format!("gagal membaca {}", path.display()))?;
    if content.trim().is_empty() {
        return Ok((
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            BalanceState::default(),
            BudgetTargetState::default(),
            default_theme(),
            default_language(),
        ));
    }

    let value: serde_json::Value = serde_json::from_str(&content)
        .with_context(|| format!("gagal mem-parse {}", path.display()))?;

    if let Some(items) = value.as_array() {
        let legacy = items
            .iter()
            .cloned()
            .map(serde_json::from_value::<LegacyExpense>)
            .collect::<std::result::Result<Vec<_>, _>>()
            .context("gagal membaca format data lama")?;

        let expenses = legacy
            .into_iter()
            .map(|item| Expense {
                id: item.id,
                date: item.date,
                category: ExpenseCategory::from_stored(&item.category),
                description: item.description,
                amount: item.amount,
            })
            .collect();

        return Ok((
            expenses,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            BalanceState::default(),
            BudgetTargetState::default(),
            default_theme(),
            default_language(),
        ));
    }

    let data: AppDataFile =
        serde_json::from_value(value).context("gagal membaca format data aplikasi")?;
    Ok((
        data.expenses,
        data.savings,
        data.earnings,
        data.work_tasks,
        data.secret_notes,
        data.balance,
        parse_targets(data.targets)?,
        data.theme,
        data.language,
    ))
}

fn default_theme() -> ThemePreset {
    ThemePreset::Forest
}

fn default_language() -> LanguagePreset {
    LanguagePreset::Indonesian
}

fn parse_targets(value: serde_json::Value) -> Result<BudgetTargetState> {
    if value.is_null() {
        return Ok(BudgetTargetState::default());
    }

    if let Ok(items) = serde_json::from_value::<Vec<BudgetTarget>>(value.clone()) {
        return Ok(BudgetTargetState { items });
    }

    if let Ok(state) = serde_json::from_value::<BudgetTargetState>(value.clone()) {
        return Ok(state);
    }

    if let Ok(legacy) = serde_json::from_value::<LegacyTargets>(value) {
        let mut items = Vec::new();
        if legacy.saving_target > 0.0 {
            items.push(BudgetTarget {
                id: 1,
                title: "Target Saving".into(),
                mode: TargetMode::Saving,
                amount: legacy.saving_target,
            });
        }
        if legacy.total_balance_target > 0.0 {
            items.push(BudgetTarget {
                id: 2,
                title: "Target Total".into(),
                mode: TargetMode::TotalBalance,
                amount: legacy.total_balance_target,
            });
        }
        return Ok(BudgetTargetState { items });
    }

    Ok(BudgetTargetState::default())
}

#[derive(Debug, Deserialize)]
struct LegacyTargets {
    #[serde(default)]
    saving_target: f64,
    #[serde(default)]
    total_balance_target: f64,
}
