mod balance_form;
mod binance_footer;
mod binance_header;
mod binance_panels;
mod celebration;
mod charts;
mod footer;
mod forms;
mod header;
mod home;
mod inspector;
mod language_selector;
mod reset_modal;
mod secret_footer;
mod secret_form;
mod secret_header;
mod secret_panels;
mod sidebar;
mod table;
mod target_delete_modal;
mod target_form;
mod theme;
mod theme_selector;
mod work_charts;
mod work_delete_animation;
mod work_footer;
mod work_form;
mod work_header;
mod work_inspector;
mod work_table;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    app::App,
    state::{AppScreen, Mode},
};

pub fn render(frame: &mut Frame, app: &App) {
    let palette = theme::palette(app.theme_preset());
    theme::render_background(frame, palette);

    if app.screen() == AppScreen::Home {
        home::render_home(frame, frame.area(), app, palette);
        if app.mode() == Mode::ThemeSelect {
            theme_selector::render_theme_selector(frame, frame.area(), app, palette);
        }
        if app.mode() == Mode::LanguageSelect {
            language_selector::render_language_selector(frame, frame.area(), app, palette);
        }
        return;
    }

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(6),
            Constraint::Min(18),
            Constraint::Length(5),
        ])
        .split(frame.area());

    match app.screen() {
        AppScreen::ExpenseTracker => {
            header::render_header(frame, layout[0], app, palette);
            header::render_summary(frame, layout[1], app, palette);
            render_expense_body(frame, layout[2], app, palette);
            footer::render_footer(frame, layout[3], app, palette);
        }
        AppScreen::WorkTracker => {
            work_header::render_header(frame, layout[0], app, palette);
            work_header::render_summary(frame, layout[1], app, palette);
            render_work_body(frame, layout[2], app, palette);
            work_footer::render_footer(frame, layout[3], app, palette);
        }
        AppScreen::SecretNotes => {
            secret_header::render_header(frame, layout[0], app, palette);
            secret_header::render_summary(frame, layout[1], app, palette);
            render_secret_body(frame, layout[2], app, palette);
            secret_footer::render_footer(frame, layout[3], app, palette);
        }
        AppScreen::BinanceTracker => {
            binance_header::render_header(frame, layout[0], app, palette);
            binance_header::render_summary(frame, layout[1], app, palette);
            binance_panels::render_body(frame, layout[2], app, palette);
            binance_footer::render_footer(frame, layout[3], app, palette);
        }
        AppScreen::Home => {}
    }

    if app.mode() == Mode::ThemeSelect {
        theme_selector::render_theme_selector(frame, frame.area(), app, palette);
    }
    if app.mode() == Mode::LanguageSelect {
        language_selector::render_language_selector(frame, frame.area(), app, palette);
    }
    if app.mode() == Mode::ConfirmTargetDelete {
        target_delete_modal::render_target_delete_modal(frame, frame.area(), app, palette);
    }
    if app.celebration_active() {
        celebration::render_celebration(frame, frame.area(), app, palette);
    }
    if app.work_delete_active() {
        work_delete_animation::render_work_delete_animation(frame, frame.area(), app, palette);
    }
}

fn render_expense_body(frame: &mut Frame, area: Rect, app: &App, palette: theme::ThemePalette) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
        .split(area);

    table::render_table(frame, cols[0], app, palette);

    match app.mode() {
        Mode::AddExpense => {
            forms::render_expense_form(frame, forms::centered_rect(76, 78, area), app, palette)
        }
        Mode::AddSaving => {
            forms::render_saving_form(frame, forms::centered_rect(72, 70, area), app, palette)
        }
        Mode::AddEarning => {
            forms::render_earning_form(frame, forms::centered_rect(72, 70, area), app, palette)
        }
        Mode::AddWork => {}
        Mode::AddBalance => balance_form::render_balance_form(
            frame,
            forms::centered_rect(64, 62, area),
            app,
            palette,
        ),
        Mode::AddTarget => {
            target_form::render_target_form(frame, forms::centered_rect(66, 62, area), app, palette)
        }
        Mode::ResetAll => {
            reset_modal::render_reset_modal(frame, forms::centered_rect(60, 46, area), app, palette)
        }
        Mode::Normal
        | Mode::Filter
        | Mode::AddSecretNote
        | Mode::EditSecretNote
        | Mode::UnlockSecretNote
        | Mode::ThemeSelect
        | Mode::LanguageSelect
        | Mode::ConfirmTargetDelete => sidebar::render_sidebar(frame, cols[1], app, palette),
    }
}

fn render_work_body(frame: &mut Frame, area: Rect, app: &App, palette: theme::ThemePalette) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(36), Constraint::Percentage(64)])
        .split(area);

    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(22),
            Constraint::Percentage(24),
            Constraint::Percentage(24),
        ])
        .split(rows[0]);
    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
        .split(rows[1]);
    let bottom_right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(bottom[1]);

    work_inspector::render_inspector(frame, top[0], app, palette);
    work_charts::render_live_clock(frame, top[1], app, palette);
    work_charts::render_deadline_radar(frame, top[2], app, palette);
    work_charts::render_task_status(frame, top[3], app, palette);
    work_table::render_table(frame, bottom[0], app, palette);
    work_charts::render_performance(frame, bottom_right[0], app, palette);
    work_charts::render_task_flow(frame, bottom_right[1], app, palette);

    match app.mode() {
        Mode::AddWork => {
            work_form::render_work_form(frame, forms::centered_rect(64, 54, area), app, palette)
        }
        Mode::ResetAll => {
            reset_modal::render_reset_modal(frame, forms::centered_rect(60, 46, area), app, palette)
        }
        Mode::Normal | Mode::Filter | Mode::ThemeSelect | Mode::LanguageSelect => {}
        Mode::AddExpense
        | Mode::AddSaving
        | Mode::AddEarning
        | Mode::AddBalance
        | Mode::AddTarget
        | Mode::ConfirmTargetDelete
        | Mode::AddSecretNote
        | Mode::EditSecretNote
        | Mode::UnlockSecretNote => {}
    }
}

fn render_secret_body(frame: &mut Frame, area: Rect, app: &App, palette: theme::ThemePalette) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(36), Constraint::Percentage(64)])
        .split(area);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(76), Constraint::Percentage(24)])
        .split(cols[0]);
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(68), Constraint::Percentage(32)])
        .split(cols[1]);

    secret_panels::render_note_list(frame, left[0], app, palette);
    secret_panels::render_vault_status(frame, left[1], app, palette);
    secret_panels::render_note_preview(frame, right[0], app, palette);
    secret_panels::render_note_activity(frame, right[1], app, palette);

    match app.mode() {
        Mode::AddSecretNote => secret_form::render_secret_note_form(
            frame,
            forms::centered_rect(72, 74, area),
            app,
            palette,
        ),
        Mode::EditSecretNote => secret_form::render_secret_note_form(
            frame,
            forms::centered_rect(72, 74, area),
            app,
            palette,
        ),
        Mode::UnlockSecretNote => secret_form::render_unlock_note_form(
            frame,
            forms::centered_rect(52, 40, area),
            app,
            palette,
        ),
        Mode::ResetAll => {
            reset_modal::render_reset_modal(frame, forms::centered_rect(60, 46, area), app, palette)
        }
        Mode::Normal | Mode::Filter | Mode::ThemeSelect | Mode::LanguageSelect => {}
        Mode::AddExpense
        | Mode::AddSaving
        | Mode::AddEarning
        | Mode::AddWork
        | Mode::AddBalance
        | Mode::AddTarget
        | Mode::ConfirmTargetDelete => {}
    }
}
