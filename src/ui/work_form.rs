use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    symbols,
    widgets::{Clear, Paragraph, Wrap},
};

use crate::{app::App, state::WorkFormField};

use super::theme::{self, ThemePalette};

pub fn render_work_form(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let form = app.work_form();
    frame.render_widget(Clear, area);

    let block = theme::modal_block(if en { "Add Task" } else { "Tambah Tugas" }, palette);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let parts = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(4),
        ])
        .split(inner);

    render_input(
        frame,
        parts[0],
        if en { "Description" } else { "Deskripsi" },
        &form.description,
        form.focus == WorkFormField::Description,
        palette,
    );
    render_input(
        frame,
        parts[1],
        if en { "Date" } else { "Tanggal" },
        &form.deadline,
        form.focus == WorkFormField::Deadline,
        palette,
    );
    render_input(
        frame,
        parts[2],
        if en { "Time" } else { "Jam" },
        &form.time,
        form.focus == WorkFormField::Time,
        palette,
    );

    frame.render_widget(
        Paragraph::new(if en {
            "Use YYYY-MM-DD for date and HH:MM for time. Tab moves fields. Enter on time saves the task."
        } else {
            "Gunakan format YYYY-MM-DD untuk tanggal dan HH:MM untuk jam. Tab pindah field. Enter di jam untuk simpan tugas."
        })
        .block(theme::panel_block(
            if en { "Interaction" } else { "Interaksi" },
            palette,
            palette.warn,
            false,
        ))
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(palette.text)),
        parts[3],
    );
}

fn render_input(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    value: &str,
    active: bool,
    palette: ThemePalette,
) {
    let border = if active { palette.warn } else { palette.muted };
    frame.render_widget(
        Paragraph::new(value.to_string())
            .style(Style::default().fg(palette.text).bg(palette.surface))
            .block(
                ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_set(symbols::border::ROUNDED)
                    .title(title)
                    .border_style(Style::default().fg(border))
                    .style(Style::default().bg(palette.surface)),
            ),
        area,
    );
}
