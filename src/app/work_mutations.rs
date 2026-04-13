use anyhow::{Context, Result};
use chrono::Local;

use super::App;
use crate::{
    models::WorkTask,
    state::{Mode, WorkForm},
};

impl App {
    pub(crate) fn open_work_form(&mut self) {
        self.mode = Mode::AddWork;
        self.work_form = WorkForm::default();
        self.status = if self.is_english() {
            "Add a task description, date, and time.".into()
        } else {
            "Tambah deskripsi tugas, tanggal, dan jam.".into()
        };
    }

    pub(crate) fn submit_work_form(&mut self) -> Result<()> {
        let description = self.work_form.description.trim();
        let deadline = self.work_form.deadline.trim();
        let deadline_time = self.work_form.time.trim();

        if description.is_empty() {
            anyhow::bail!(if self.is_english() {
                "Task description cannot be empty"
            } else {
                "Deskripsi tugas tidak boleh kosong"
            });
        }
        chrono::NaiveDate::parse_from_str(deadline, "%Y-%m-%d").context(if self.is_english() {
            "Deadline must use YYYY-MM-DD format"
        } else {
            "Deadline harus memakai format YYYY-MM-DD"
        })?;
        chrono::NaiveTime::parse_from_str(deadline_time, "%H:%M").context(
            if self.is_english() {
                "Time must use HH:MM format"
            } else {
                "Jam harus memakai format HH:MM"
            },
        )?;

        let task = WorkTask {
            id: self.next_work_id,
            description: description.to_string(),
            deadline: deadline.to_string(),
            deadline_time: deadline_time.to_string(),
            completed: false,
            created_at: Local::now().format("%Y-%m-%d %H:%M").to_string(),
            completed_at: None,
        };

        self.next_work_id += 1;
        self.work_tasks.push(task);
        self.persist()?;
        self.work_form = WorkForm::default();
        self.mode = Mode::Normal;
        self.work_selected = 0;
        self.clamp_selection();
        self.status = if self.is_english() {
            "Task added successfully.".into()
        } else {
            "Tugas berhasil ditambahkan.".into()
        };
        Ok(())
    }

    pub(crate) fn delete_selected_work(&mut self) -> Result<()> {
        let indices = self.filtered_work_indices();
        if indices.is_empty() {
            self.status = if self.is_english() {
                "No task can be deleted.".into()
            } else {
                "Tidak ada tugas yang bisa dihapus.".into()
            };
            return Ok(());
        }

        let idx = indices[self.work_selected];
        let removed = self.work_tasks.remove(idx);
        let removed_description = removed.description.clone();
        let removed_was_completed = removed.completed;
        self.persist()?;
        self.clamp_selection();
        self.work_delete_frames_left = 18;
        self.work_delete_task_title = removed_description.clone();
        self.work_delete_was_completed = removed_was_completed;
        self.status = if removed_was_completed {
            if self.is_english() {
                format!("Completed task '{}' archived.", removed_description)
            } else {
                format!("Tugas selesai '{}' diarsipkan.", removed_description)
            }
        } else {
            if self.is_english() {
                format!("Task '{}' deleted before completion.", removed_description)
            } else {
                format!("Tugas '{}' dihapus sebelum selesai.", removed_description)
            }
        };
        Ok(())
    }

    pub(crate) fn toggle_selected_work_completion(&mut self) -> Result<()> {
        let indices = self.filtered_work_indices();
        if indices.is_empty() {
            self.status = if self.is_english() {
                "No task is selected.".into()
            } else {
                "Tidak ada tugas yang dipilih.".into()
            };
            return Ok(());
        }

        let idx = indices[self.work_selected];
        let task = &mut self.work_tasks[idx];
        task.completed = !task.completed;
        task.completed_at = if task.completed {
            Some(Local::now().format("%Y-%m-%d %H:%M").to_string())
        } else {
            None
        };
        let description = task.description.clone();
        let completed = task.completed;
        self.persist()?;
        self.clamp_selection();
        self.status = if completed {
            if self.is_english() {
                format!("Task '{}' marked as done.", description)
            } else {
                format!("Tugas '{}' ditandai selesai.", description)
            }
        } else if self.is_english() {
            format!("Task '{}' returned to pending.", description)
        } else {
            format!("Tugas '{}' dikembalikan ke pending.", description)
        };
        Ok(())
    }
}
