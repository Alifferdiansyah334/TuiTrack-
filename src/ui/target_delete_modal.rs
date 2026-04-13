use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

use crate::{app::App, state::ConfirmChoice};

use super::{
    forms,
    theme::{self, ThemePalette},
};

pub fn render_target_delete_modal(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let popup = forms::centered_rect(52, 36, area);
    frame.render_widget(Clear, popup);

    let block = theme::modal_block(if en { "Delete Target" } else { "Hapus Target" }, palette);
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(1),
        ])
        .margin(1)
        .split(inner);

    let title = app
        .focused_target()
        .map(|target| target.title.clone())
        .unwrap_or_else(|| {
            if en {
                "this target".into()
            } else {
                "target ini".into()
            }
        });

    frame.render_widget(
        Paragraph::new(if en {
            format!("Delete target '{}'?", title)
        } else {
            format!("Hapus target '{}'?", title)
        })
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(palette.text)
                .add_modifier(Modifier::BOLD),
        ),
        rows[0],
    );

    let choices = Line::from(vec![
        confirm_chip(
            if en { "Yes" } else { "Ya" },
            app.confirm_choice() == ConfirmChoice::Yes,
            palette,
        ),
        Span::raw("   "),
        confirm_chip(
            if en { "No" } else { "Tidak" },
            app.confirm_choice() == ConfirmChoice::No,
            palette,
        ),
    ]);
    frame.render_widget(
        Paragraph::new(choices).alignment(Alignment::Center),
        rows[1],
    );

    frame.render_widget(
        Paragraph::new(if en {
            "Enter confirms, Esc cancels"
        } else {
            "Enter konfirmasi, Esc batal"
        })
        .alignment(Alignment::Center)
        .style(Style::default().fg(palette.muted)),
        rows[2],
    );
}

fn confirm_chip(label: &str, active: bool, palette: ThemePalette) -> Span<'static> {
    if active {
        Span::styled(
            format!(" {label} "),
            Style::default()
                .fg(Color::Black)
                .bg(palette.accent)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(
            format!(" {label} "),
            Style::default().fg(palette.muted).bg(palette.surface),
        )
    }
}
