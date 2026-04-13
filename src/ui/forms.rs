use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Clear, Paragraph, Wrap},
};

use crate::{
    app::App,
    models::CATEGORY_OPTIONS,
    state::{EarningFormField, ExpenseForm, ExpenseFormField, SavingFormField},
};

use super::theme::{self, ThemePalette};

pub fn render_expense_form(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let form = app.expense_form();
    frame.render_widget(Clear, area);

    let block = theme::modal_block(
        if en {
            "Add Expense"
        } else {
            "Tambah Pengeluaran"
        },
        palette,
    );
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let parts = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(4),
        ])
        .split(inner);

    render_input(
        frame,
        parts[0],
        if en { "Date" } else { "Tanggal" },
        &form.date,
        form.focus == ExpenseFormField::Date,
        palette,
    );
    render_category_input(frame, parts[1], form, app.language_preset(), palette);
    render_input(
        frame,
        parts[2],
        if en { "Description" } else { "Deskripsi" },
        &form.description,
        form.focus == ExpenseFormField::Description,
        palette,
    );
    render_input(
        frame,
        parts[3],
        if en { "Amount" } else { "Nominal" },
        &form.amount,
        form.focus == ExpenseFormField::Amount,
        palette,
    );

    let help = Paragraph::new(vec![
        Line::styled(
            if en {
                "Left/right changes category. Tab moves fields."
            } else {
                "Kiri/kanan untuk kategori. Tab pindah field."
            },
            Style::default().fg(palette.text),
        ),
        Line::styled(
            if en {
                "Press Enter on amount to save. Esc cancels."
            } else {
                "Enter di nominal untuk simpan. Esc untuk batal."
            },
            Style::default().fg(palette.muted),
        ),
    ])
    .block(theme::panel_block(
        if en { "Interaction" } else { "Interaksi" },
        palette,
        palette.warn,
        false,
    ))
    .wrap(Wrap { trim: true });
    frame.render_widget(help, parts[4]);
}

pub fn render_saving_form(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let form = app.saving_form();
    frame.render_widget(Clear, area);

    let block = theme::modal_block(if en { "Add Saving" } else { "Tambah Nabung" }, palette);
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
        if en { "Date" } else { "Tanggal" },
        &form.date,
        form.focus == SavingFormField::Date,
        palette,
    );
    render_input(
        frame,
        parts[1],
        if en { "Description" } else { "Deskripsi" },
        &form.description,
        form.focus == SavingFormField::Description,
        palette,
    );
    render_input(
        frame,
        parts[2],
        if en { "Amount" } else { "Nominal" },
        &form.amount,
        form.focus == SavingFormField::Amount,
        palette,
    );

    let help = Paragraph::new(vec![
        Line::styled(
            if en {
                "Tab switches fields."
            } else {
                "Tab untuk pindah field."
            },
            Style::default().fg(palette.text),
        ),
        Line::styled(
            if en {
                "Press Enter on amount to save. Esc cancels."
            } else {
                "Enter di nominal untuk simpan. Esc untuk batal."
            },
            Style::default().fg(palette.muted),
        ),
    ])
    .block(theme::panel_block(
        if en { "Interaction" } else { "Interaksi" },
        palette,
        palette.warn,
        false,
    ))
    .wrap(Wrap { trim: true });
    frame.render_widget(help, parts[3]);
}

pub fn render_earning_form(frame: &mut Frame, area: Rect, app: &App, palette: ThemePalette) {
    let en = app.is_english();
    let form = app.earning_form();
    frame.render_widget(Clear, area);

    let block = theme::modal_block(
        if en {
            "Add Earning"
        } else {
            "Tambah Pemasukan"
        },
        palette,
    );
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
        if en { "Date" } else { "Tanggal" },
        &form.date,
        form.focus == EarningFormField::Date,
        palette,
    );
    render_input(
        frame,
        parts[1],
        if en { "Description" } else { "Deskripsi" },
        &form.description,
        form.focus == EarningFormField::Description,
        palette,
    );
    render_input(
        frame,
        parts[2],
        if en { "Amount" } else { "Nominal" },
        &form.amount,
        form.focus == EarningFormField::Amount,
        palette,
    );

    let help = Paragraph::new(vec![
        Line::styled(
            if en {
                "Tab switches fields."
            } else {
                "Tab untuk pindah field."
            },
            Style::default().fg(palette.text),
        ),
        Line::styled(
            if en {
                "Press Enter on amount to save. Esc cancels."
            } else {
                "Enter di nominal untuk simpan. Esc untuk batal."
            },
            Style::default().fg(palette.muted),
        ),
    ])
    .block(theme::panel_block(
        if en { "Interaction" } else { "Interaksi" },
        palette,
        palette.warn,
        false,
    ))
    .wrap(Wrap { trim: true });
    frame.render_widget(help, parts[3]);
}

pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
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
    let widget = Paragraph::new(value.to_string())
        .style(Style::default().fg(palette.text).bg(palette.surface))
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .title(title)
                .border_style(Style::default().fg(border))
                .style(Style::default().bg(palette.surface)),
        );
    frame.render_widget(widget, area);
}

fn render_category_input(
    frame: &mut Frame,
    area: Rect,
    form: &ExpenseForm,
    language: crate::state::LanguagePreset,
    palette: ThemePalette,
) {
    let border = if form.focus == ExpenseFormField::Category {
        palette.warn
    } else {
        palette.muted
    };
    let line = Line::from(
        CATEGORY_OPTIONS
            .iter()
            .enumerate()
            .flat_map(|(idx, category)| {
                let selected = idx == form.category_index;
                let span = if selected {
                    Span::styled(
                        format!(" {} ", category.label(language)),
                        Style::default()
                            .fg(Color::Black)
                            .bg(palette.accent)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::styled(
                        format!(" {} ", category.label(language)),
                        Style::default().fg(palette.muted).bg(palette.surface),
                    )
                };
                [span, Span::raw(" ")]
            })
            .collect::<Vec<_>>(),
    );

    let widget = Paragraph::new(line)
        .style(Style::default().fg(palette.text).bg(palette.surface))
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .title(if language.is_english() {
                    "Category"
                } else {
                    "Kategori"
                })
                .border_style(Style::default().fg(border))
                .style(Style::default().bg(palette.surface)),
        );
    frame.render_widget(widget, area);
}
