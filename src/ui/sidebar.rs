use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::app::App;

use super::{charts, inspector, theme::ThemePalette};

pub fn render_sidebar(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let parts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(34), Constraint::Percentage(66)])
        .split(area);

    inspector::render_inspector(frame, parts[0], app, palette);
    charts::render_charts(frame, parts[1], app, palette);
}
