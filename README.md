# TuiTrack

TuiTrack is a terminal-based personal tracker built with Rust, `ratatui`, and `crossterm`.

It combines several modules in one TUI application:

- expense tracking
- savings and earnings tracking
- work task tracking
- budget target tracking
- secret note storage
- Binance market and account dashboard

## Requirements

- Rust toolchain
- terminal that supports alternate screen mode

## Run Locally

```bash
cargo run
```

The app reads and writes local data to `expenses.json`. That file is intentionally ignored by git so personal data does not get committed.

## Environment Variables

Create a `.env` file if you want to use the Binance integration:

```env
BINANCE_API_KEY=your_api_key
BINANCE_API_SECRET=your_api_secret
```

You can start from `.env.example`.

## Notes

- If `expenses.json` does not exist yet, the app starts with empty data.
- `.env` and `expenses.json` are excluded from version control for safety.

## Build Check

```bash
cargo check
```
