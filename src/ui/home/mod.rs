mod deck;
mod hero;
mod util;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::app::App;

use super::theme::{self, ThemePalette};

pub fn render_home(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let shell = util::centered_rect(92, 90, area);
    let shell_block = theme::panel_block("TuiTrack Launcher", palette, palette.accent, true);
    let shell_inner = shell_block.inner(shell);
    frame.render_widget(shell_block, shell);

    let tick = app.animation_tick() as usize;
    let english = app.language_preset().is_english();
    util::render_ambient(frame, shell_inner, palette, tick);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(8),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(19),
            Constraint::Length(4),
            Constraint::Min(2),
        ])
        .margin(1)
        .split(shell_inner);

    hero::render_statusline(frame, rows[0], palette, tick, english);
    hero::render_banner(frame, rows[1], palette, tick);
    hero::render_logo(frame, rows[2], shell_inner.width, palette, tick);
    hero::render_subtitle(frame, rows[3], palette, english);
    hero::render_feature_ribbon(frame, rows[4], palette, tick, english);
    deck::render_launchdeck(frame, rows[5], app, palette, tick);
    deck::render_boot_strip(frame, rows[6], app, palette, tick);
    hero::render_help(frame, rows[7], palette, english);
}
