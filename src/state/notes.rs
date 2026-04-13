#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecretNoteFormField {
    Title,
    Content,
    Passkey,
}

impl SecretNoteFormField {
    pub fn next(self) -> Self {
        match self {
            Self::Title => Self::Content,
            Self::Content => Self::Passkey,
            Self::Passkey => Self::Passkey,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Title => Self::Title,
            Self::Content => Self::Title,
            Self::Passkey => Self::Content,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SecretNoteForm {
    pub title: String,
    pub content: String,
    pub passkey: String,
    pub focus: SecretNoteFormField,
}

impl Default for SecretNoteForm {
    fn default() -> Self {
        Self {
            title: String::new(),
            content: String::new(),
            passkey: String::new(),
            focus: SecretNoteFormField::Title,
        }
    }
}

impl SecretNoteForm {
    pub fn current_value_mut(&mut self) -> &mut String {
        match self.focus {
            SecretNoteFormField::Title => &mut self.title,
            SecretNoteFormField::Content => &mut self.content,
            SecretNoteFormField::Passkey => &mut self.passkey,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UnlockNoteForm {
    pub passkey: String,
}
