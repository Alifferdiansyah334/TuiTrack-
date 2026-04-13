use super::App;
use crate::models::SecretNote;

impl App {
    pub fn filtered_secret_note_indices(&self) -> Vec<usize> {
        self.secret_notes
            .iter()
            .enumerate()
            .filter_map(|(idx, note)| {
                note.matches_filter(
                    &self.filter,
                    self.unlocked_notes.get(&note.id).map(String::as_str),
                )
                .then_some(idx)
            })
            .collect()
    }

    pub fn secret_note_at(&self, idx: usize) -> Option<&SecretNote> {
        self.secret_notes.get(idx)
    }

    pub fn selected_secret_note(&self) -> Option<&SecretNote> {
        self.filtered_secret_note_indices()
            .get(self.secret_selected)
            .and_then(|idx| self.secret_notes.get(*idx))
    }

    pub fn unlocked_secret_content(&self, note_id: u64) -> Option<&str> {
        self.unlocked_notes.get(&note_id).map(String::as_str)
    }

    pub fn secret_note_count(&self) -> usize {
        self.secret_notes.len()
    }

    pub fn unlocked_secret_note_count(&self) -> usize {
        self.unlocked_notes.len()
    }

    pub fn locked_secret_note_count(&self) -> usize {
        self.secret_note_count()
            .saturating_sub(self.unlocked_secret_note_count())
    }

    pub fn secret_note_payload_chars(&self) -> usize {
        self.unlocked_notes
            .values()
            .map(|content| content.chars().count())
            .sum()
    }

    pub fn selected_secret_note_is_unlocked(&self) -> bool {
        self.selected_secret_note()
            .is_some_and(|note| self.unlocked_notes.contains_key(&note.id))
    }
}
