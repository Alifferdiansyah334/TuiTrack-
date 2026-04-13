use chrono::Local;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkFormField {
    Description,
    Deadline,
    Time,
}

impl WorkFormField {
    pub fn next(self) -> Self {
        match self {
            Self::Description => Self::Deadline,
            Self::Deadline => Self::Time,
            Self::Time => Self::Time,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Description => Self::Description,
            Self::Deadline => Self::Description,
            Self::Time => Self::Deadline,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkForm {
    pub description: String,
    pub deadline: String,
    pub time: String,
    pub focus: WorkFormField,
}

impl Default for WorkForm {
    fn default() -> Self {
        Self {
            description: String::new(),
            deadline: Local::now().format("%Y-%m-%d").to_string(),
            time: "17:00".into(),
            focus: WorkFormField::Description,
        }
    }
}

impl WorkForm {
    pub fn current_value_mut(&mut self) -> &mut String {
        match self.focus {
            WorkFormField::Description => &mut self.description,
            WorkFormField::Deadline => &mut self.deadline,
            WorkFormField::Time => &mut self.time,
        }
    }
}
