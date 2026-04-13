use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::app::App;

use super::{theme::ThemePalette, work_charts, work_inspector};

pub fn render_sidebar(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let parts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(28), Constraint::Percentage(72)])
        .split(area);

    work_inspector::render_inspector(frame, parts[0], app, palette);
    work_charts::render_charts(frame, parts[1], app, palette);
}
