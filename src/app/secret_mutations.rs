use anyhow::Result;
use chrono::Local;

use super::App;
use crate::{
    models::SecretNote,
    security::{decrypt_note, encrypt_note, hash_passkey},
    state::{Mode, SecretNoteForm, UnlockNoteForm},
};

impl App {
    pub(crate) fn open_secret_note_form(&mut self) {
        self.mode = Mode::AddSecretNote;
        self.editing_secret_note_id = None;
        self.secret_note_form = SecretNoteForm::default();
        self.status = if self.is_english() {
            "Add a vault title, note content, and a dedicated passkey.".into()
        } else {
            "Isi judul vault, isi note, dan passkey khusus untuk note ini.".into()
        };
    }

    pub(crate) fn submit_secret_note_form(&mut self) -> Result<()> {
        let title = self.secret_note_form.title.trim();
        let content = self.secret_note_form.content.trim();
        let passkey = self.secret_note_form.passkey.trim();

        if title.is_empty() {
            anyhow::bail!(if self.is_english() {
                "Secret note title cannot be empty"
            } else {
                "Judul catatan rahasia tidak boleh kosong"
            });
        }
        if content.is_empty() {
            anyhow::bail!(if self.is_english() {
                "Secret note content cannot be empty"
            } else {
                "Isi catatan rahasia tidak boleh kosong"
            });
        }
        if passkey.len() < 4 {
            anyhow::bail!(if self.is_english() {
                "Passkey must contain at least 4 characters"
            } else {
                "Passkey harus minimal 4 karakter"
            });
        }

        let created_at = Local::now().format("%Y-%m-%d %H:%M").to_string();
        let salt = format!(
            "note-{}-{}",
            self.next_secret_note_id,
            Local::now().timestamp_micros()
        );
        let note = SecretNote {
            id: self.next_secret_note_id,
            title: title.to_string(),
            encrypted_content: encrypt_note(passkey, &salt, content),
            passkey_hash: hash_passkey(&salt, passkey),
            salt,
            created_at,
            last_unlocked_at: None,
        };

        self.next_secret_note_id += 1;
        self.secret_notes.insert(0, note);
        self.persist()?;
        self.secret_selected = 0;
        self.mode = Mode::Normal;
        self.secret_note_form = SecretNoteForm::default();
        self.clamp_selection();
        self.status = if self.is_english() {
            "Secret note stored in vault.".into()
        } else {
            "Catatan rahasia berhasil disimpan ke vault.".into()
        };
        Ok(())
    }

    pub(crate) fn open_edit_secret_note_form(&mut self) {
        let Some((note_id, note_title)) = self
            .selected_secret_note()
            .map(|note| (note.id, note.title.clone()))
        else {
            self.status = if self.is_english() {
                "No secret note is selected.".into()
            } else {
                "Tidak ada catatan rahasia yang dipilih.".into()
            };
            return;
        };

        let Some(content) = self.unlocked_notes.get(&note_id).cloned() else {
            self.status = if self.is_english() {
                "Unlock the secret note before editing it.".into()
            } else {
                "Buka dulu catatan rahasia sebelum mengeditnya.".into()
            };
            return;
        };

        self.mode = Mode::EditSecretNote;
        self.editing_secret_note_id = Some(note_id);
        self.secret_note_form = SecretNoteForm {
            title: note_title,
            content,
            passkey: String::new(),
            focus: crate::state::SecretNoteFormField::Title,
        };
        self.status = if self.is_english() {
            "Edit the title or content, then enter the note passkey to save changes.".into()
        } else {
            "Edit judul atau isi, lalu masukkan passkey note untuk menyimpan perubahan.".into()
        };
    }

    pub(crate) fn submit_edit_secret_note_form(&mut self) -> Result<()> {
        let Some(note_id) = self.editing_secret_note_id else {
            anyhow::bail!(if self.is_english() {
                "No secret note is being edited"
            } else {
                "Tidak ada catatan rahasia yang sedang diedit"
            });
        };

        let title = self.secret_note_form.title.trim();
        let content = self.secret_note_form.content.trim();
        let passkey = self.secret_note_form.passkey.trim();

        if title.is_empty() {
            anyhow::bail!(if self.is_english() {
                "Secret note title cannot be empty"
            } else {
                "Judul catatan rahasia tidak boleh kosong"
            });
        }
        if content.is_empty() {
            anyhow::bail!(if self.is_english() {
                "Secret note content cannot be empty"
            } else {
                "Isi catatan rahasia tidak boleh kosong"
            });
        }
        if passkey.len() < 4 {
            anyhow::bail!(if self.is_english() {
                "Passkey must contain at least 4 characters"
            } else {
                "Passkey harus minimal 4 karakter"
            });
        }

        let Some(index) = self.secret_notes.iter().position(|note| note.id == note_id) else {
            anyhow::bail!(if self.is_english() {
                "Secret note is no longer available"
            } else {
                "Catatan rahasia sudah tidak tersedia"
            });
        };

        let note = &mut self.secret_notes[index];
        if hash_passkey(&note.salt, passkey) != note.passkey_hash {
            anyhow::bail!(if self.is_english() {
                "Passkey does not match this secret note"
            } else {
                "Passkey tidak cocok untuk catatan rahasia ini"
            });
        }

        note.title = title.to_string();
        note.encrypted_content = encrypt_note(passkey, &note.salt, content);
        note.last_unlocked_at = Some(Local::now().format("%Y-%m-%d %H:%M").to_string());
        self.unlocked_notes.insert(note_id, content.to_string());

        let title = note.title.clone();
        self.persist()?;
        self.mode = Mode::Normal;
        self.editing_secret_note_id = None;
        self.secret_note_form = SecretNoteForm::default();
        self.status = if self.is_english() {
            format!("Secret note '{}' updated.", title)
        } else {
            format!("Catatan rahasia '{}' berhasil diperbarui.", title)
        };
        Ok(())
    }

    pub(crate) fn open_unlock_note_form(&mut self) {
        let Some((note_id, note_title)) = self
            .selected_secret_note()
            .map(|note| (note.id, note.title.clone()))
        else {
            self.status = if self.is_english() {
                "No secret note is selected.".into()
            } else {
                "Tidak ada catatan rahasia yang dipilih.".into()
            };
            return;
        };

        if self.unlocked_notes.contains_key(&note_id) {
            self.unlocked_notes.remove(&note_id);
            self.status = if self.is_english() {
                format!("Secret note '{}' locked again.", note_title)
            } else {
                format!("Catatan rahasia '{}' dikunci lagi.", note_title)
            };
            return;
        }

        self.pending_unlock_note_id = Some(note_id);
        self.editing_secret_note_id = None;
        self.unlock_note_form = UnlockNoteForm::default();
        self.mode = Mode::UnlockSecretNote;
        self.status = if self.is_english() {
            format!("Enter passkey for '{}'.", note_title)
        } else {
            format!("Masukkan passkey untuk '{}'.", note_title)
        };
    }

    pub(crate) fn submit_unlock_note_form(&mut self) -> Result<()> {
        let Some(note_id) = self.pending_unlock_note_id else {
            anyhow::bail!(if self.is_english() {
                "No secret note is waiting to be unlocked"
            } else {
                "Tidak ada catatan rahasia yang sedang menunggu dibuka"
            });
        };

        let passkey = self.unlock_note_form.passkey.trim();
        if passkey.is_empty() {
            anyhow::bail!(if self.is_english() {
                "Passkey cannot be empty"
            } else {
                "Passkey tidak boleh kosong"
            });
        }

        let Some(index) = self.secret_notes.iter().position(|note| note.id == note_id) else {
            anyhow::bail!(if self.is_english() {
                "Secret note is no longer available"
            } else {
                "Catatan rahasia sudah tidak tersedia"
            });
        };

        let note = &mut self.secret_notes[index];
        if hash_passkey(&note.salt, passkey) != note.passkey_hash {
            anyhow::bail!(if self.is_english() {
                "Passkey does not match this secret note"
            } else {
                "Passkey tidak cocok untuk catatan rahasia ini"
            });
        }

        let content = decrypt_note(passkey, &note.salt, &note.encrypted_content)?;
        note.last_unlocked_at = Some(Local::now().format("%Y-%m-%d %H:%M").to_string());
        let title = note.title.clone();
        self.unlocked_notes.insert(note_id, content);
        self.persist()?;
        self.mode = Mode::Normal;
        self.pending_unlock_note_id = None;
        self.editing_secret_note_id = None;
        self.unlock_note_form = UnlockNoteForm::default();
        self.status = if self.is_english() {
            format!("Secret note '{}' unlocked.", title)
        } else {
            format!("Catatan rahasia '{}' berhasil dibuka.", title)
        };
        Ok(())
    }

    pub(crate) fn delete_selected_secret_note(&mut self) -> Result<()> {
        let indices = self.filtered_secret_note_indices();
        if indices.is_empty() {
            self.status = if self.is_english() {
                "No secret note can be deleted.".into()
            } else {
                "Tidak ada catatan rahasia yang bisa dihapus.".into()
            };
            return Ok(());
        }

        let idx = indices[self.secret_selected];
        let removed = self.secret_notes.remove(idx);
        self.unlocked_notes.remove(&removed.id);
        if self.editing_secret_note_id == Some(removed.id) {
            self.editing_secret_note_id = None;
        }
        self.persist()?;
        self.clamp_selection();
        self.status = if self.is_english() {
            format!("Secret note '{}' deleted from vault.", removed.title)
        } else {
            format!("Catatan rahasia '{}' dihapus dari vault.", removed.title)
        };
        Ok(())
    }
}
