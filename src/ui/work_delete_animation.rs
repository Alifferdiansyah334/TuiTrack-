use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Clear, Paragraph},
};

use crate::app::App;

use super::{
    forms,
    theme::{self, ThemePalette},
};

pub fn render_work_delete_animation(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    palette: ThemePalette,
) {
    let en = app.is_english();
    let completed = app.work_delete_was_completed();
    let popup = forms::centered_rect(62, 36, area);
    frame.render_widget(Clear, popup);

    let block = theme::modal_block(
        if completed {
            if en {
                "Task Archived"
            } else {
                "Tugas Diarsipkan"
            }
        } else if en {
            "Task Dropped"
        } else {
            "Tugas Dihapus"
        },
        palette,
    );
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .margin(1)
        .split(inner);

    let phase = (app.animation_tick() % 4) as usize;
    let (burst, pulse, face, headline, subline, accent) = if completed {
        let burst = [
            "* * *   * * *",
            "+ + +   + + +",
            "o o o   o o o",
            "x x x   x x x",
        ][phase];
        let pulse = [
            "ARCHIVE LOCKED",
            "ARCHIVE SEALED",
            "ARCHIVE LOCKED",
            "ARCHIVE SEALED",
        ][phase];
        let face = ["( ^_^ )", "( ^o^ )", "( ^-^ )", "( ^_^ )/"][phase];
        let headline = if en {
            "DONE TASK CLEARED"
        } else {
            "TUGAS SELESAI DIBERSIHKAN"
        };
        let subline = if en {
            [
                "clean finish stored in the archive",
                "clean finish stored in the archive!",
                "clean finish stored in the archive!!",
                "clean finish stored in the archive!",
            ][phase]
        } else {
            [
                "selesai rapi dan masuk arsip",
                "selesai rapi dan masuk arsip!",
                "selesai rapi dan masuk arsip!!",
                "selesai rapi dan masuk arsip!",
            ][phase]
        };
        (burst, pulse, face, headline, subline, palette.accent)
    } else {
        let burst = ["\\\\  \\\\  \\\\", "vv  vv  vv", "..  ..  ..", "__  __  __"][phase];
        let pulse = ["DROP ALERT", "EARLY REMOVE", "DROP ALERT", "EARLY REMOVE"][phase];
        let face = ["( o_o )", "( -_- )", "( ._. )", "( o_o )/"][phase];
        let headline = if en {
            "TASK REMOVED EARLY"
        } else {
            "TUGAS DIHAPUS LEBIH AWAL"
        };
        let subline = if en {
            [
                "removed before completion",
                "removed before completion.",
                "removed before completion..",
                "removed before completion...",
            ][phase]
        } else {
            [
                "dihapus sebelum selesai",
                "dihapus sebelum selesai.",
                "dihapus sebelum selesai..",
                "dihapus sebelum selesai...",
            ][phase]
        };
        (burst, pulse, face, headline, subline, palette.danger)
    };

    frame.render_widget(
        Paragraph::new(burst)
            .alignment(Alignment::Center)
            .style(Style::default().fg(accent).add_modifier(Modifier::BOLD)),
        rows[0],
    );
    frame.render_widget(
        Paragraph::new(pulse).alignment(Alignment::Center).style(
            Style::default()
                .fg(palette.warn)
                .add_modifier(Modifier::BOLD),
        ),
        rows[1],
    );
    frame.render_widget(
        Paragraph::new(face)
            .alignment(Alignment::Center)
            .style(Style::default().fg(accent).add_modifier(Modifier::BOLD)),
        rows[2],
    );
    frame.render_widget(
        Paragraph::new(headline)
            .alignment(Alignment::Center)
            .style(Style::default().fg(accent).add_modifier(Modifier::BOLD)),
        rows[3],
    );
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(format!("'{}'", app.work_delete_task_title())),
            Line::from(subline),
        ])
        .alignment(Alignment::Center)
        .style(Style::default().fg(palette.text)),
        rows[4],
    );
}
