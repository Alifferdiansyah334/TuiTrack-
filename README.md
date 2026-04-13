# TuiTrack

TuiTrack is a terminal-first personal dashboard built with Rust, `ratatui`, and `crossterm`.

It combines budgeting, work tracking, private note storage, and Binance monitoring in a single text user interface.

## Highlights

- Track expenses, savings, earnings, and current balance in one place
- Manage budget targets for savings goals or total balance goals
- Monitor work tasks from the same interface
- Store encrypted secret notes locally
- View Binance watchlist data, chart intervals, and spot account snapshots
- Switch theme and language directly from the app

## Modules

### Expense Tracker

- Record expenses with category, date, description, and amount
- Switch between expense, saving, and earning modes
- Review totals, balance impact, and stored entries

### Work Tracker

- Add and update work items
- Track task progress from the terminal UI

### Secret Notes

- Save sensitive notes in encrypted form
- Lock and unlock notes with a passkey

### Binance Tracker

- Load live watchlist data for major symbols such as `BTCUSDT`, `ETHUSDT`, and `SOLUSDT`
- Change chart interval directly from the UI
- Show spot account balances when API credentials are available

## Tech Stack

- Rust
- `ratatui`
- `crossterm`
- `reqwest`
- `serde`
- `dotenvy`

## Requirements

- Rust toolchain installed
- A terminal that supports alternate screen mode

## Getting Started

Clone the repository and run:

```bash
cargo run
```

For a quick validation build:

```bash
cargo check
```

## Environment Setup

Binance account features require a local `.env` file:

```env
BINANCE_API_KEY=your_api_key
BINANCE_API_SECRET=your_api_secret
```

You can start from:

```bash
cp .env.example .env
```

If `.env` is missing, the app can still run. Public Binance market data remains available, but private account data will not load.

## Data Storage

TuiTrack reads and writes its local application data to:

```text
expenses.json
```

Behavior:

- If `expenses.json` does not exist, the app starts with empty data
- `.env`, `.env.*`, and `expenses.json` are excluded from git
- `.env.example` is included as a safe template

## Keyboard Shortcuts

Common controls used across the app:

- `Enter` to open a module or confirm an action
- Arrow keys and `j` / `k` to move through panels or items
- `Tab` and `Shift+Tab` to change focus in forms
- `/` to filter or search in supported views
- `c` to clear filters
- `t` to change theme
- `l` to change language
- `q` to return to the home screen

Shortcut hints are also shown inside the footer of the application.

## Project Structure

```text
src/
  app/        application behavior and actions
  state/      form and screen state
  ui/         ratatui rendering
  binance.rs  Binance integration
  storage.rs  local persistence
```

## Notes

- This project is designed for local-first personal use
- Sensitive runtime data should stay in local files, not in version control
- For public sharing, keep `.env` and `expenses.json` private
