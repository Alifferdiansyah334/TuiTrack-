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

pub fn render_celebration(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let popup = forms::centered_rect(60, 34, area);
    frame.render_widget(Clear, popup);

    let block = theme::modal_block(
        if app.celebration_success() {
            if en {
                "Target Cleared"
            } else {
                "Target Dibersihkan"
            }
        } else {
            if en {
                "Target Removed"
            } else {
                "Target Dihapus"
            }
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
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .margin(1)
        .split(inner);

    let burst = if app.celebration_success() {
        match app.animation_tick() % 4 {
            0 => "*  *  *  *  *",
            1 => "+  +  +  +  +",
            2 => "x  x  x  x  x",
            _ => "o  o  o  o  o",
        }
    } else {
        match app.animation_tick() % 4 {
            0 => ".  .  .  .  .",
            1 => "_  _  _  _  _",
            2 => ",  ,  ,  ,  ,",
            _ => ";  ;  ;  ;  ;",
        }
    };
    let face = if app.celebration_success() {
        match app.animation_tick() % 4 {
            0 => "( ^_^ )",
            1 => "( ^o^ )",
            2 => "( ^-^ )",
            _ => "( ^_^ )/",
        }
    } else {
        match app.animation_tick() % 4 {
            0 => "( T_T )",
            1 => "( ;_; )",
            2 => "( ._. )",
            _ => "( T_T )/",
        }
    };
    let headline = if app.celebration_success() {
        if en {
            "TARGET COMPLETED"
        } else {
            "TARGET TERCAPAI"
        }
    } else {
        if en {
            "TARGET NOT COMPLETED"
        } else {
            "TARGET BELUM TERCAPAI"
        }
    };
    let subline = if app.celebration_success() {
        if en {
            match app.animation_tick() % 4 {
                0 => "completed and cleared successfully",
                1 => "completed and cleared successfully!",
                2 => "completed and cleared successfully!!",
                _ => "completed and cleared successfully!",
            }
        } else {
            match app.animation_tick() % 4 {
                0 => "selesai dan berhasil dibersihkan",
                1 => "selesai dan berhasil dibersihkan!",
                2 => "selesai dan berhasil dibersihkan!!",
                _ => "selesai dan berhasil dibersihkan!",
            }
        }
    } else {
        if en {
            match app.animation_tick() % 4 {
                0 => "removed before reaching the target",
                1 => "removed before reaching the target.",
                2 => "removed before reaching the target..",
                _ => "removed before reaching the target...",
            }
        } else {
            match app.animation_tick() % 4 {
                0 => "dihapus sebelum target tercapai",
                1 => "dihapus sebelum target tercapai.",
                2 => "dihapus sebelum target tercapai..",
                _ => "dihapus sebelum target tercapai...",
            }
        }
    };

    frame.render_widget(
        Paragraph::new(burst).alignment(Alignment::Center).style(
            Style::default()
                .fg(if app.celebration_success() {
                    palette.warn
                } else {
                    palette.muted
                })
                .add_modifier(Modifier::BOLD),
        ),
        rows[0],
    );
    frame.render_widget(
        Paragraph::new(face).alignment(Alignment::Center).style(
            Style::default()
                .fg(if app.celebration_success() {
                    palette.accent
                } else {
                    palette.danger
                })
                .add_modifier(Modifier::BOLD),
        ),
        rows[1],
    );
    frame.render_widget(
        Paragraph::new(headline).alignment(Alignment::Center).style(
            Style::default()
                .fg(if app.celebration_success() {
                    palette.accent
                } else {
                    palette.danger
                })
                .add_modifier(Modifier::BOLD),
        ),
        rows[2],
    );
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(format!("'{}'", app.celebration_target_title())),
            Line::from(subline),
        ])
        .alignment(Alignment::Center)
        .style(Style::default().fg(palette.text)),
        rows[3],
    );
}
