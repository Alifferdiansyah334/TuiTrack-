use anyhow::{Context, Result};

use super::App;
use crate::{
    formatting::format_currency,
    models::{EarningEntry, Expense, SavingEntry, TrackerMode},
    state::{
        BalanceForm, BudgetTarget, ConfirmChoice, EarningForm, ExpenseForm, LanguagePreset, Mode,
        SavingForm, TargetForm, ThemePreset,
    },
    storage,
};

impl App {
    pub(crate) fn submit_expense_form(&mut self) -> Result<()> {
        let date = self.expense_form.date.trim();
        let description = self.expense_form.description.trim();
        let amount = self
            .expense_form
            .amount
            .trim()
            .replace(',', ".")
            .parse::<f64>()
            .context(if self.is_english() {
                "Expense amount must be a number"
            } else {
                "Nominal pengeluaran harus berupa angka"
            })?;

        if date.is_empty() {
            anyhow::bail!(if self.is_english() {
                "Expense date cannot be empty"
            } else {
                "Tanggal pengeluaran tidak boleh kosong"
            });
        }
        if description.is_empty() {
            anyhow::bail!(if self.is_english() {
                "Expense description cannot be empty"
            } else {
                "Deskripsi pengeluaran tidak boleh kosong"
            });
        }
        if amount <= 0.0 {
            anyhow::bail!(if self.is_english() {
                "Expense amount must be greater than 0"
            } else {
                "Nominal pengeluaran harus lebih besar dari 0"
            });
        }

        let expense = Expense {
            id: self.next_expense_id,
            date: date.to_string(),
            category: self.expense_form.category(),
            description: description.to_string(),
            amount,
        };

        self.next_expense_id += 1;
        self.expenses.insert(0, expense);
        self.persist()?;
        self.expense_selected = 0;
        self.mode = Mode::Normal;
        self.expense_form = ExpenseForm::default();
        self.clamp_selection();
        self.status = if self.is_english() {
            "Expense added successfully.".into()
        } else {
            "Pengeluaran berhasil ditambahkan.".into()
        };
        Ok(())
    }

    pub(crate) fn submit_saving_form(&mut self) -> Result<()> {
        let date = self.saving_form.date.trim();
        let description = self.saving_form.description.trim();
        let amount = self
            .saving_form
            .amount
            .trim()
            .replace(',', ".")
            .parse::<f64>()
            .context(if self.is_english() {
                "Saving amount must be a number"
            } else {
                "Nominal tabungan harus berupa angka"
            })?;

        if date.is_empty() {
            anyhow::bail!(if self.is_english() {
                "Saving date cannot be empty"
            } else {
                "Tanggal tabungan tidak boleh kosong"
            });
        }
        if description.is_empty() {
            anyhow::bail!(if self.is_english() {
                "Saving description cannot be empty"
            } else {
                "Deskripsi tabungan tidak boleh kosong"
            });
        }
        if amount <= 0.0 {
            anyhow::bail!(if self.is_english() {
                "Saving amount must be greater than 0"
            } else {
                "Nominal tabungan harus lebih besar dari 0"
            });
        }

        let saving = SavingEntry {
            id: self.next_saving_id,
            date: date.to_string(),
            description: description.to_string(),
            amount,
        };

        self.next_saving_id += 1;
        self.savings.insert(0, saving);
        self.persist()?;
        self.saving_selected = 0;
        self.mode = Mode::Normal;
        self.saving_form = SavingForm::default();
        self.clamp_selection();
        self.status = if self.is_english() {
            "Saving entry added successfully.".into()
        } else {
            "Data nabung berhasil ditambahkan.".into()
        };
        Ok(())
    }

    pub(crate) fn submit_earning_form(&mut self) -> Result<()> {
        let date = self.earning_form.date.trim();
        let description = self.earning_form.description.trim();
        let amount = self
            .earning_form
            .amount
            .trim()
            .replace(',', ".")
            .parse::<f64>()
            .context(if self.is_english() {
                "Earning amount must be a number"
            } else {
                "Nominal pemasukan harus berupa angka"
            })?;

        if date.is_empty() {
            anyhow::bail!(if self.is_english() {
                "Earning date cannot be empty"
            } else {
                "Tanggal pemasukan tidak boleh kosong"
            });
        }
        if description.is_empty() {
            anyhow::bail!(if self.is_english() {
                "Earning description cannot be empty"
            } else {
                "Deskripsi pemasukan tidak boleh kosong"
            });
        }
        if amount <= 0.0 {
            anyhow::bail!(if self.is_english() {
                "Earning amount must be greater than 0"
            } else {
                "Nominal pemasukan harus lebih besar dari 0"
            });
        }

        let earning = EarningEntry {
            id: self.next_earning_id,
            date: date.to_string(),
            description: description.to_string(),
            amount,
        };

        self.next_earning_id += 1;
        self.earnings.insert(0, earning);
        self.persist()?;
        self.earning_selected = 0;
        self.mode = Mode::Normal;
        self.earning_form = EarningForm::default();
        self.clamp_selection();
        self.status = if self.is_english() {
            "Earning entry added successfully.".into()
        } else {
            "Data pemasukan berhasil ditambahkan.".into()
        };
        Ok(())
    }

    pub(crate) fn submit_balance_form(&mut self) -> Result<()> {
        let amount = if self.balance_form.amount.trim().is_empty() {
            self.balance.amount
        } else {
            self.balance_form
                .amount
                .trim()
                .replace(',', ".")
                .parse::<f64>()
                .context(if self.is_english() {
                    "Balance amount must be a number"
                } else {
                    "Nominal balance harus berupa angka"
                })?
        };

        if amount < 0.0 {
            anyhow::bail!(if self.is_english() {
                "Balance amount cannot be negative"
            } else {
                "Nominal balance tidak boleh negatif"
            });
        }

        self.balance.enabled = self.balance_form.enabled;
        self.balance.amount = amount;
        self.persist()?;
        self.mode = Mode::Normal;
        self.balance_form = BalanceForm::default();
        self.status = if self.balance.enabled {
            if self.is_english() {
                format!(
                    "Balance enabled. Base balance set to {}.",
                    format_currency(self.balance.amount)
                )
            } else {
                format!(
                    "Balance aktif. Balance dasar diset ke {}.",
                    format_currency(self.balance.amount)
                )
            }
        } else {
            if self.is_english() {
                "Base balance disabled. Calculation returns to savings + earnings - expense.".into()
            } else {
                "Balance dasar dimatikan. Kalkulasi kembali ke tabungan + pemasukan - expense."
                    .into()
            }
        };
        Ok(())
    }

    pub(crate) fn submit_target_form(&mut self) -> Result<()> {
        let title = self.target_form.title.trim().to_string();
        let target = self
            .target_form
            .amount
            .trim()
            .replace(',', ".")
            .parse::<f64>()
            .context(if self.is_english() {
                "Target amount must be a number"
            } else {
                "Nominal target harus berupa angka"
            })?;

        if target <= 0.0 {
            anyhow::bail!(if self.is_english() {
                "Target amount must be greater than 0"
            } else {
                "Nominal target harus lebih besar dari 0"
            });
        }
        if title.is_empty() {
            anyhow::bail!(if self.is_english() {
                "Target title cannot be empty"
            } else {
                "Judul target tidak boleh kosong"
            });
        }

        let entry = BudgetTarget {
            id: self.next_target_id,
            title: title.clone(),
            mode: self.target_form.mode,
            amount: target,
        };
        self.next_target_id += 1;
        self.targets.items.insert(0, entry);
        self.target_page = 0;

        self.persist()?;
        self.mode = Mode::Normal;
        self.target_form = TargetForm::default();
        self.status = if self.is_english() {
            format!("Target '{}' saved.", title)
        } else {
            format!("Target '{}' disimpan.", title)
        };
        Ok(())
    }

    pub(crate) fn delete_selected(&mut self) -> Result<()> {
        match self.tracker_mode {
            TrackerMode::Expense => {
                let indices = self.filtered_expense_indices();
                if indices.is_empty() {
                    self.status = if self.is_english() {
                        "No expense entry can be deleted.".into()
                    } else {
                        "Tidak ada pengeluaran yang bisa dihapus.".into()
                    };
                    return Ok(());
                }
                let idx = indices[self.expense_selected];
                let removed = self.expenses.remove(idx);
                self.persist()?;
                self.clamp_selection();
                self.status = if self.is_english() {
                    format!("Expense '{}' deleted.", removed.description)
                } else {
                    format!("Pengeluaran '{}' dihapus.", removed.description)
                };
            }
            TrackerMode::Saving => {
                let indices = self.filtered_saving_indices();
                if indices.is_empty() {
                    self.status = if self.is_english() {
                        "No saving entry can be deleted.".into()
                    } else {
                        "Tidak ada data nabung yang bisa dihapus.".into()
                    };
                    return Ok(());
                }
                let idx = indices[self.saving_selected];
                let removed = self.savings.remove(idx);
                self.persist()?;
                self.clamp_selection();
                self.status = if self.is_english() {
                    format!("Saving entry '{}' deleted.", removed.description)
                } else {
                    format!("Data nabung '{}' dihapus.", removed.description)
                };
            }
            TrackerMode::Earning => {
                let indices = self.filtered_earning_indices();
                if indices.is_empty() {
                    self.status = if self.is_english() {
                        "No earning entry can be deleted.".into()
                    } else {
                        "Tidak ada data pemasukan yang bisa dihapus.".into()
                    };
                    return Ok(());
                }
                let idx = indices[self.earning_selected];
                let removed = self.earnings.remove(idx);
                self.persist()?;
                self.clamp_selection();
                self.status = if self.is_english() {
                    format!("Earning entry '{}' deleted.", removed.description)
                } else {
                    format!("Data pemasukan '{}' dihapus.", removed.description)
                };
            }
        }
        Ok(())
    }

    pub(crate) fn switch_tracker_mode(&mut self, mode: TrackerMode) {
        self.tracker_mode = mode;
        self.mode = Mode::Normal;
        self.clamp_selection();
        self.status = if self.is_english() {
            format!(
                "Switched to {} mode.",
                self.tracker_mode.label(self.language_preset)
            )
        } else {
            format!(
                "Pindah ke mode {}.",
                self.tracker_mode.label(self.language_preset)
            )
        };
    }

    pub(crate) fn open_theme_selector(&mut self) {
        self.mode = Mode::ThemeSelect;
        self.theme_selected = ThemePreset::ALL
            .iter()
            .position(|preset| *preset == self.theme_preset)
            .unwrap_or(0);
        self.status = if self.is_english() {
            "Choose a theme with j/k then press Enter.".into()
        } else {
            "Pilih theme dengan j/k lalu Enter.".into()
        };
    }

    pub(crate) fn open_language_selector(&mut self) {
        self.mode = Mode::LanguageSelect;
        self.language_selected = LanguagePreset::ALL
            .iter()
            .position(|preset| *preset == self.language_preset)
            .unwrap_or(0);
        self.status = if self.is_english() {
            "Choose a language with j/k then press Enter.".into()
        } else {
            "Pilih bahasa dengan j/k lalu Enter.".into()
        };
    }

    pub(crate) fn open_balance_form(&mut self) {
        self.mode = Mode::AddBalance;
        self.balance_form = BalanceForm {
            enabled: self.balance.enabled,
            amount: if self.balance.amount > 0.0 {
                self.balance.amount.to_string()
            } else {
                String::new()
            },
            focus: crate::state::BalanceFormField::Enabled,
        };
        self.status = if self.is_english() {
            "Set base balance and adjust the balance amount.".into()
        } else {
            "Atur balance dasar dan set nominal balance.".into()
        };
    }

    pub(crate) fn open_target_form(&mut self) {
        self.mode = Mode::AddTarget;
        self.target_form = TargetForm::default();
        self.status = if self.is_english() {
            "Fill the target title, choose the type, then enter the amount.".into()
        } else {
            "Isi judul target, pilih jenis, lalu masukkan nominal.".into()
        };
    }

    pub(crate) fn next_target_page(&mut self) {
        let pages = self.target_page_count();
        if pages <= 1 {
            self.status = if self.is_english() {
                "There is no other target page yet.".into()
            } else {
                "Belum ada halaman target lain.".into()
            };
            return;
        }

        self.target_page = (self.target_page + 1) % pages;
        self.status = if self.is_english() {
            format!(
                "Switched to target page {}/{}.",
                self.target_page + 1,
                pages
            )
        } else {
            format!("Pindah ke target page {}/{}.", self.target_page + 1, pages)
        };
    }

    pub(crate) fn delete_pending_target(&mut self) -> Result<()> {
        let Some(target_id) = self.pending_delete_target_id else {
            self.mode = Mode::Normal;
            self.status = if self.is_english() {
                "No target is selected.".into()
            } else {
                "Tidak ada target yang dipilih.".into()
            };
            return Ok(());
        };

        let Some(index) = self
            .targets
            .items
            .iter()
            .position(|item| item.id == target_id)
        else {
            self.mode = Mode::Normal;
            self.pending_delete_target_id = None;
            self.status = if self.is_english() {
                "Target is no longer available.".into()
            } else {
                "Target sudah tidak tersedia.".into()
            };
            return Ok(());
        };

        let completed = {
            let target = &self.targets.items[index];
            let (current, _, _, _) = self.target_progress(target);
            current >= target.amount
        };
        let removed = self.targets.items.remove(index);

        let pages = self.target_page_count();
        if self.target_page >= pages {
            self.target_page = pages.saturating_sub(1);
        }

        self.persist()?;
        self.mode = Mode::Normal;
        self.pending_delete_target_id = None;
        self.confirm_choice = ConfirmChoice::No;

        if completed {
            self.celebration_frames_left = 18;
            self.celebration_target_title = removed.title.clone();
            self.celebration_success = true;
            self.status = if self.is_english() {
                format!("Target '{}' completed and archived.", removed.title)
            } else {
                format!("Target '{}' selesai dan diarsipkan.", removed.title)
            };
        } else {
            self.celebration_frames_left = 18;
            self.celebration_target_title = removed.title.clone();
            self.celebration_success = false;
            self.status = if self.is_english() {
                format!("Target '{}' deleted.", removed.title)
            } else {
                format!("Target '{}' dihapus.", removed.title)
            };
        }
        Ok(())
    }

    pub(crate) fn persist(&mut self) -> Result<()> {
        storage::save_data(
            &self.data_path,
            &self.expenses,
            &self.savings,
            &self.earnings,
            &self.work_tasks,
            &self.secret_notes,
            &self.balance,
            &self.targets,
            self.theme_preset,
            self.language_preset,
        )?;
        self.reset_animation();
        Ok(())
    }

    fn reset_animation(&mut self) {
        self.animation_tick = 0;
        self.animation_frames_left = super::ANIMATION_FRAMES;
    }
}
