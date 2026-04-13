use super::App;
use crate::state::Mode;

impl App {
    pub fn is_english(&self) -> bool {
        self.language_preset.is_english()
    }

    pub fn mode_hint(&self) -> &'static str {
        if self.screen == crate::state::AppScreen::Home {
            return match self.mode {
                Mode::ThemeSelect => self.hint_theme_selector(),
                Mode::LanguageSelect => self.hint_language_selector(),
                _ => {
                    if self.is_english() {
                        "j/k select menu, Enter open, t theme, l language, q quit"
                    } else {
                        "j/k pilih menu, Enter buka, t theme, l bahasa, q keluar"
                    }
                }
            };
        }

        if self.screen == crate::state::AppScreen::WorkTracker {
            return match self.mode {
                Mode::Normal => {
                    if self.is_english() {
                        "j/k panel, arrows table, a add, x done, d delete, q home"
                    } else {
                        "j/k panel, panah tabel, a tambah, x selesai, d hapus, q home"
                    }
                }
                Mode::Filter => {
                    if self.is_english() {
                        "Type filter, Enter finish, Esc cancel"
                    } else {
                        "Ketik filter, Enter selesai, Esc batal"
                    }
                }
                Mode::AddWork => {
                    if self.is_english() {
                        "Tab switch field, Enter on deadline to save"
                    } else {
                        "Tab pindah field, Enter di deadline untuk simpan"
                    }
                }
                Mode::ResetAll => {
                    if self.is_english() {
                        "Type /resetall then Enter, Esc cancel"
                    } else {
                        "Ketik /resetall lalu Enter, Esc batal"
                    }
                }
                Mode::ThemeSelect => self.hint_theme_selector(),
                Mode::LanguageSelect => self.hint_language_selector(),
                _ => {
                    if self.is_english() {
                        "use arrows and shortcuts"
                    } else {
                        "pakai panah dan shortcut"
                    }
                }
            };
        }

        if self.screen == crate::state::AppScreen::SecretNotes {
            return match self.mode {
                Mode::Normal => {
                    if self.is_english() {
                        "arrows move, a add, e edit, u or Enter unlock/lock, d delete, q home"
                    } else {
                        "panah pindah, a tambah, e edit, u atau Enter buka/kunci, d hapus, q home"
                    }
                }
                Mode::Filter => {
                    if self.is_english() {
                        "Type filter, Enter finish, Esc cancel"
                    } else {
                        "Ketik filter, Enter selesai, Esc batal"
                    }
                }
                Mode::AddSecretNote => {
                    if self.is_english() {
                        "Tab switch field, Enter on passkey to save"
                    } else {
                        "Tab pindah field, Enter di passkey untuk simpan"
                    }
                }
                Mode::EditSecretNote => {
                    if self.is_english() {
                        "Unlock first, then Tab fields and Enter on passkey to save edits"
                    } else {
                        "Buka dulu, lalu Tab field dan Enter di passkey untuk simpan edit"
                    }
                }
                Mode::UnlockSecretNote => {
                    if self.is_english() {
                        "Type passkey, Enter unlock, Esc cancel"
                    } else {
                        "Ketik passkey, Enter buka, Esc batal"
                    }
                }
                Mode::ResetAll => {
                    if self.is_english() {
                        "Type /resetall then Enter, Esc cancel"
                    } else {
                        "Ketik /resetall lalu Enter, Esc batal"
                    }
                }
                Mode::ThemeSelect => self.hint_theme_selector(),
                Mode::LanguageSelect => self.hint_language_selector(),
                _ => {
                    if self.is_english() {
                        "use vault shortcuts"
                    } else {
                        "pakai shortcut vault"
                    }
                }
            };
        }

        if self.screen == crate::state::AppScreen::BinanceTracker {
            return match self.mode {
                Mode::Normal => {
                    if self.is_english() {
                        "j/k panel, arrows list, Enter load chart, left/right interval, u refresh, q home"
                    } else {
                        "j/k panel, panah list, Enter muat chart, kiri/kanan interval, u refresh, q home"
                    }
                }
                Mode::ThemeSelect => self.hint_theme_selector(),
                Mode::LanguageSelect => self.hint_language_selector(),
                _ => {
                    if self.is_english() {
                        "use Binance tracker shortcuts"
                    } else {
                        "pakai shortcut Binance tracker"
                    }
                }
            };
        }

        match self.mode {
            Mode::Normal => {
                if self.is_english() {
                    "1/2/3 mode, j/k panel, arrows table, Enter target, g add target, p page, q home"
                } else {
                    "1/2/3 mode, j/k panel, panah tabel, Enter target, g tambah target, p page, q home"
                }
            }
            Mode::Filter => {
                if self.is_english() {
                    "Type filter, Enter finish, Esc cancel"
                } else {
                    "Ketik filter, Enter selesai, Esc batal"
                }
            }
            Mode::AddExpense => {
                if self.is_english() {
                    "Tab switch field, left/right choose category, Enter save"
                } else {
                    "Tab pindah field, kiri/kanan pilih kategori, Enter simpan"
                }
            }
            Mode::AddSaving => {
                if self.is_english() {
                    "Tab switch field, Enter on amount to save"
                } else {
                    "Tab pindah field, Enter di nominal untuk simpan"
                }
            }
            Mode::AddEarning => {
                if self.is_english() {
                    "Tab switch field, Enter on amount to save"
                } else {
                    "Tab pindah field, Enter di nominal untuk simpan"
                }
            }
            Mode::AddWork => {
                if self.is_english() {
                    "Tab switch field, Enter on deadline to save"
                } else {
                    "Tab pindah field, Enter di deadline untuk simpan"
                }
            }
            Mode::AddSecretNote => {
                if self.is_english() {
                    "Tab switch field, Enter on passkey to save"
                } else {
                    "Tab pindah field, Enter di passkey untuk simpan"
                }
            }
            Mode::EditSecretNote => {
                if self.is_english() {
                    "Unlock first, then Tab fields and Enter on passkey to save edits"
                } else {
                    "Buka dulu, lalu Tab field dan Enter di passkey untuk simpan edit"
                }
            }
            Mode::UnlockSecretNote => {
                if self.is_english() {
                    "Type passkey, Enter unlock, Esc cancel"
                } else {
                    "Ketik passkey, Enter buka, Esc batal"
                }
            }
            Mode::AddBalance => {
                if self.is_english() {
                    "Tab switch field, left/right toggle, Enter save"
                } else {
                    "Tab pindah field, kiri/kanan toggle, Enter simpan"
                }
            }
            Mode::AddTarget => {
                if self.is_english() {
                    "Tab switch field, left/right choose target, Enter save"
                } else {
                    "Tab pindah field, kiri/kanan pilih target, Enter simpan"
                }
            }
            Mode::ResetAll => {
                if self.is_english() {
                    "Type /resetall then Enter, Esc cancel"
                } else {
                    "Ketik /resetall lalu Enter, Esc batal"
                }
            }
            Mode::ConfirmTargetDelete => {
                if self.is_english() {
                    "Left/right choose Yes/No, Enter confirm, Esc cancel"
                } else {
                    "Kiri/kanan pilih Ya/Tidak, Enter konfirmasi, Esc batal"
                }
            }
            Mode::ThemeSelect => self.hint_theme_selector(),
            Mode::LanguageSelect => self.hint_language_selector(),
        }
    }

    pub fn hint_theme_selector(&self) -> &'static str {
        if self.is_english() {
            "j/k choose theme, Enter apply, Esc cancel"
        } else {
            "j/k pilih theme, Enter terapkan, Esc batal"
        }
    }

    pub fn hint_language_selector(&self) -> &'static str {
        if self.is_english() {
            "j/k choose language, Enter apply, Esc cancel"
        } else {
            "j/k pilih bahasa, Enter terapkan, Esc batal"
        }
    }

    pub fn status_cancelled(&self, subject_id: &str) -> String {
        let subject = match (subject_id, self.is_english()) {
            ("target_form", true) => "target setup",
            ("target_form", false) => "atur target",
            ("reset", true) => "data reset",
            ("reset", false) => "reset data",
            ("balance", true) => "balance setup",
            ("balance", false) => "atur balance",
            ("expense_form", true) => "add expense",
            ("expense_form", false) => "tambah pengeluaran",
            ("saving_form", true) => "add saving",
            ("saving_form", false) => "tambah nabung",
            ("earning_form", true) => "add earning",
            ("earning_form", false) => "tambah pemasukan",
            ("work_form", true) => "add task",
            ("work_form", false) => "tambah tugas",
            ("secret_note_form", true) => "add secret note",
            ("secret_note_form", false) => "tambah catatan rahasia",
            ("edit_secret_note_form", true) => "edit secret note",
            ("edit_secret_note_form", false) => "edit catatan rahasia",
            ("unlock_note_form", true) => "unlock secret note",
            ("unlock_note_form", false) => "buka catatan rahasia",
            ("theme_selector", true) => "theme selector",
            ("theme_selector", false) => "theme selector",
            ("language_selector", true) => "language selector",
            ("language_selector", false) => "language selector",
            ("target_delete", true) => "target deletion",
            ("target_delete", false) => "hapus target",
            _ => "action",
        };
        if self.is_english() {
            format!("{subject} cancelled.")
        } else {
            format!("{subject} dibatalkan.")
        }
    }
}
