mod app;
mod binance;
mod formatting;
mod models;
mod security;
mod state;
mod storage;
mod ui;

use std::{
    io::{self, Stdout},
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::app::App;

const DATA_FILE: &str = "expenses.json";

fn main() -> Result<()> {
    load_dotenv();
    let data_path = std::env::current_dir()
        .context("gagal membaca current directory")?
        .join(DATA_FILE);
    let mut terminal = setup_terminal()?;
    let result = run_app(&mut terminal, data_path);
    restore_terminal(&mut terminal)?;
    result
}

fn load_dotenv() {
    if let Ok(current_dir) = std::env::current_dir() {
        let current_env = current_dir.join(".env");
        if current_env.is_file() {
            let _ = dotenvy::from_path_override(current_env);
            return;
        }
    }

    let manifest_env = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    if manifest_env.is_file() {
        let _ = dotenvy::from_path_override(manifest_env);
        return;
    }

    let Ok(executable) = std::env::current_exe() else {
        let _ = dotenvy::dotenv_override();
        return;
    };

    for directory in executable.ancestors().filter(|path| path.is_dir()) {
        let candidate = directory.join(".env");
        if candidate.is_file() {
            let _ = dotenvy::from_path_override(&candidate);
            return;
        }
    }

    let _ = dotenvy::dotenv_override();
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode().context("gagal mengaktifkan raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("gagal masuk alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("gagal membuat terminal")
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("gagal menonaktifkan raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("gagal keluar dari alternate screen")?;
    terminal.show_cursor().context("gagal menampilkan cursor")
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, data_path: PathBuf) -> Result<()> {
    let mut app = App::load(data_path)?;

    loop {
        app.tick();
        terminal.draw(|frame| ui::render(frame, &app))?;

        if event::poll(Duration::from_millis(120))? {
            let Event::Key(key) = event::read()? else {
                continue;
            };

            if key.kind != KeyEventKind::Press {
                continue;
            }

            match app.handle_key(key) {
                Ok(true) => break,
                Ok(false) => {}
                Err(err) => app.set_error(err.to_string()),
            }
        }
    }

    Ok(())
}
