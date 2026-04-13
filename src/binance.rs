use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result};
use chrono::Local;
use hmac::{Hmac, Mac};
use reqwest::blocking::Client;
use serde::Deserialize;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub const WATCHLIST: [&str; 5] = ["BTCUSDT", "ETHUSDT", "BNBUSDT", "SOLUSDT", "XRPUSDT"];
pub const INTERVALS: [&str; 4] = ["15m", "1h", "4h", "1d"];
pub const DEFAULT_INTERVAL_INDEX: usize = 1;

const DEFAULT_PUBLIC_BASE_URL: &str = "https://data-api.binance.vision";
const DEFAULT_PRIVATE_BASE_URL: &str = "https://api.binance.com";

#[derive(Debug, Clone)]
pub struct MarketTicker {
    pub symbol: String,
    pub last_price: f64,
    pub price_change_percent: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub volume: f64,
    pub quote_volume: f64,
}

#[derive(Debug, Clone)]
pub struct KlinePoint {
    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Clone)]
pub struct AccountBalance {
    pub asset: String,
    pub free: f64,
    pub locked: f64,
    pub total: f64,
    pub quote_value: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum AccountState {
    Connected { key_hint: String },
    MissingCredentials,
    Error(String),
}

impl Default for AccountState {
    fn default() -> Self {
        Self::MissingCredentials
    }
}

impl AccountState {
    pub fn title(&self, english: bool) -> &'static str {
        match self {
            Self::Connected { .. } => {
                if english {
                    "Connected"
                } else {
                    "Terhubung"
                }
            }
            Self::MissingCredentials => {
                if english {
                    "No Credentials"
                } else {
                    "Tanpa Kredensial"
                }
            }
            Self::Error(_) => {
                if english {
                    "Account Error"
                } else {
                    "Error Akun"
                }
            }
        }
    }

    pub fn detail(&self, english: bool) -> String {
        match self {
            Self::Connected { key_hint } => {
                if english {
                    format!("Spot account loaded with API key {key_hint}.")
                } else {
                    format!("Akun spot berhasil dimuat dengan API key {key_hint}.")
                }
            }
            Self::MissingCredentials => {
                if english {
                    "Set BINANCE_API_KEY and BINANCE_API_SECRET in your environment or .env file."
                        .into()
                } else {
                    "Isi BINANCE_API_KEY dan BINANCE_API_SECRET di environment atau file .env."
                        .into()
                }
            }
            Self::Error(message) => {
                if english {
                    format!("Binance account request failed: {message}")
                } else {
                    format!("Request akun Binance gagal: {message}")
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinanceDashboard {
    pub symbol: String,
    pub interval: String,
    pub tickers: Vec<MarketTicker>,
    pub klines: Vec<KlinePoint>,
    pub balances: Vec<AccountBalance>,
    pub wallet_estimate_usdt: Option<f64>,
    pub account_state: AccountState,
    pub last_updated: Option<String>,
}

impl Default for BinanceDashboard {
    fn default() -> Self {
        Self {
            symbol: WATCHLIST[0].to_string(),
            interval: INTERVALS[DEFAULT_INTERVAL_INDEX].to_string(),
            tickers: Vec::new(),
            klines: Vec::new(),
            balances: Vec::new(),
            wallet_estimate_usdt: None,
            account_state: AccountState::MissingCredentials,
            last_updated: None,
        }
    }
}

impl BinanceDashboard {
    pub fn ticker_for(&self, symbol: &str) -> Option<&MarketTicker> {
        self.tickers.iter().find(|ticker| ticker.symbol == symbol)
    }
}

pub fn watchlist_index(symbol: &str) -> usize {
    WATCHLIST
        .iter()
        .position(|item| *item == symbol)
        .unwrap_or(0)
}

pub fn interval_index(interval: &str) -> usize {
    INTERVALS
        .iter()
        .position(|item| *item == interval)
        .unwrap_or(0)
}

pub fn fetch_dashboard(symbol: &str, interval: &str) -> Result<BinanceDashboard> {
    let client = Client::builder()
        .danger_accept_invalid_certs(insecure_tls_enabled())
        .timeout(Duration::from_secs(10))
        .user_agent("tuitrack-binance/0.1")
        .build()
        .context("gagal membuat HTTP client Binance")?;

    let tickers = fetch_watchlist_tickers(&client)?;
    let klines = fetch_klines(&client, symbol, interval)?;

    let (balances, wallet_estimate_usdt, account_state) = match load_credentials() {
        Some(credentials) => match fetch_account_balances(&client, &credentials, &tickers) {
            Ok((balances, wallet_estimate_usdt)) => (
                balances,
                wallet_estimate_usdt,
                AccountState::Connected {
                    key_hint: mask_key(&credentials.api_key),
                },
            ),
            Err(err) => (Vec::new(), None, AccountState::Error(err.to_string())),
        },
        None => (Vec::new(), None, AccountState::MissingCredentials),
    };

    Ok(BinanceDashboard {
        symbol: symbol.to_string(),
        interval: interval.to_string(),
        tickers,
        klines,
        balances,
        wallet_estimate_usdt,
        account_state,
        last_updated: Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
    })
}

fn fetch_watchlist_tickers(client: &Client) -> Result<Vec<MarketTicker>> {
    let symbols = WATCHLIST
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>();
    let payload = serde_json::to_string(&symbols).context("gagal membuat payload watchlist")?;
    let raw = client
        .get(format!("{}/api/v3/ticker/24hr", public_base_url()))
        .query(&[("symbols", payload)])
        .send()
        .context("gagal mengambil data watchlist Binance")?
        .error_for_status()
        .context("response watchlist Binance tidak sukses")?
        .json::<Vec<RawTicker24hr>>()
        .context("gagal parse data watchlist Binance")?;

    let mut tickers = raw
        .into_iter()
        .map(|item| MarketTicker {
            symbol: item.symbol,
            last_price: parse_number(&item.last_price),
            price_change_percent: parse_number(&item.price_change_percent),
            high_price: parse_number(&item.high_price),
            low_price: parse_number(&item.low_price),
            volume: parse_number(&item.volume),
            quote_volume: parse_number(&item.quote_volume),
        })
        .collect::<Vec<_>>();

    tickers.sort_by_key(|ticker| watchlist_index(&ticker.symbol));
    Ok(tickers)
}

fn fetch_klines(client: &Client, symbol: &str, interval: &str) -> Result<Vec<KlinePoint>> {
    let rows = client
        .get(format!("{}/api/v3/klines", public_base_url()))
        .query(&[
            ("symbol", symbol.to_string()),
            ("interval", interval.to_string()),
            ("limit", "32".to_string()),
        ])
        .send()
        .with_context(|| format!("gagal mengambil kline Binance untuk {symbol}"))?
        .error_for_status()
        .context("response kline Binance tidak sukses")?
        .json::<Vec<Vec<serde_json::Value>>>()
        .context("gagal parse data kline Binance")?;

    let points = rows
        .into_iter()
        .filter_map(|row| parse_kline_row(&row))
        .collect::<Vec<_>>();
    Ok(points)
}

fn fetch_account_balances(
    client: &Client,
    credentials: &Credentials,
    tickers: &[MarketTicker],
) -> Result<(Vec<AccountBalance>, Option<f64>)> {
    let timestamp = unix_timestamp_ms();
    let query = format!("omitZeroBalances=true&recvWindow=5000&timestamp={timestamp}");
    let signature = sign_query(&query, &credentials.api_secret)?;
    let url = format!(
        "{}/api/v3/account?{query}&signature={signature}",
        private_base_url()
    );

    let response = client
        .get(url)
        .header("X-MBX-APIKEY", &credentials.api_key)
        .send()
        .context("gagal menghubungi endpoint akun Binance")?
        .error_for_status()
        .context("response akun Binance tidak sukses")?
        .json::<RawAccount>()
        .context("gagal parse data akun Binance")?;

    let mut balances = response
        .balances
        .into_iter()
        .filter_map(|item| {
            let free = parse_number(&item.free);
            let locked = parse_number(&item.locked);
            let total = free + locked;
            (total > 0.0).then_some(AccountBalance {
                asset: item.asset,
                free,
                locked,
                total,
                quote_value: None,
            })
        })
        .collect::<Vec<_>>();

    let mut wallet_estimate_usdt = 0.0;
    let mut has_estimate = false;

    for balance in &mut balances {
        let estimate = estimate_quote_value(client, balance, tickers);
        if let Some(value) = estimate {
            wallet_estimate_usdt += value;
            has_estimate = true;
        }
        balance.quote_value = estimate;
    }

    balances.sort_by(|left, right| {
        right
            .quote_value
            .unwrap_or(0.0)
            .total_cmp(&left.quote_value.unwrap_or(0.0))
            .then_with(|| right.total.total_cmp(&left.total))
            .then_with(|| left.asset.cmp(&right.asset))
    });

    Ok((balances, has_estimate.then_some(wallet_estimate_usdt)))
}

fn estimate_quote_value(
    client: &Client,
    balance: &AccountBalance,
    tickers: &[MarketTicker],
) -> Option<f64> {
    if is_stablecoin(&balance.asset) {
        return Some(balance.total);
    }

    let spot_symbol = format!("{}USDT", balance.asset);
    if let Some(ticker) = tickers.iter().find(|ticker| ticker.symbol == spot_symbol) {
        return Some(balance.total * ticker.last_price);
    }

    fetch_symbol_price(client, &spot_symbol)
        .ok()
        .map(|price| balance.total * price)
}

fn fetch_symbol_price(client: &Client, symbol: &str) -> Result<f64> {
    let response = client
        .get(format!("{}/api/v3/ticker/price", public_base_url()))
        .query(&[("symbol", symbol.to_string())])
        .send()
        .with_context(|| format!("gagal mengambil harga {symbol}"))?;

    if !response.status().is_success() {
        anyhow::bail!("harga {symbol} tidak tersedia");
    }

    let raw = response
        .json::<RawPriceTicker>()
        .with_context(|| format!("gagal parse harga {symbol}"))?;
    Ok(parse_number(&raw.price))
}

fn load_credentials() -> Option<Credentials> {
    let api_key = std::env::var("BINANCE_API_KEY")
        .ok()
        .filter(|value| !value.trim().is_empty());
    let api_secret = std::env::var("BINANCE_API_SECRET")
        .ok()
        .filter(|value| !value.trim().is_empty());

    if let (Some(api_key), Some(api_secret)) = (api_key, api_secret) {
        return Some(Credentials {
            api_key,
            api_secret,
        });
    }

    load_credentials_from_dotenv()
}

fn public_base_url() -> String {
    std::env::var("BINANCE_PUBLIC_BASE_URL").unwrap_or_else(|_| DEFAULT_PUBLIC_BASE_URL.into())
}

fn private_base_url() -> String {
    std::env::var("BINANCE_PRIVATE_BASE_URL").unwrap_or_else(|_| DEFAULT_PRIVATE_BASE_URL.into())
}

fn insecure_tls_enabled() -> bool {
    std::env::var("BINANCE_INSECURE_TLS")
        .ok()
        .is_some_and(|value| matches!(value.trim(), "1" | "true" | "TRUE" | "yes" | "YES"))
}

fn load_credentials_from_dotenv() -> Option<Credentials> {
    for path in dotenv_candidates() {
        let content = fs::read_to_string(&path).ok()?;
        let api_key = dotenv_value(&content, "BINANCE_API_KEY");
        let api_secret = dotenv_value(&content, "BINANCE_API_SECRET");
        if let (Some(api_key), Some(api_secret)) = (api_key, api_secret) {
            return Some(Credentials {
                api_key,
                api_secret,
            });
        }
    }

    None
}

fn dotenv_candidates() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(current_dir) = std::env::current_dir() {
        push_candidate(&mut paths, current_dir.join(".env"));
    }

    push_candidate(
        &mut paths,
        Path::new(env!("CARGO_MANIFEST_DIR")).join(".env"),
    );

    if let Ok(executable) = std::env::current_exe() {
        for directory in executable.ancestors().filter(|path| path.is_dir()) {
            push_candidate(&mut paths, directory.join(".env"));
        }
    }

    paths
}

fn push_candidate(paths: &mut Vec<PathBuf>, candidate: PathBuf) {
    if candidate.is_file() && !paths.iter().any(|path| path == &candidate) {
        paths.push(candidate);
    }
}

fn dotenv_value(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let (entry_key, entry_value) = trimmed.split_once('=')?;
        if entry_key.trim() != key {
            continue;
        }

        let value = entry_value
            .trim()
            .trim_matches('"')
            .trim_matches('\'')
            .to_string();
        if !value.is_empty() {
            return Some(value);
        }
    }

    None
}

fn sign_query(query: &str, secret: &str) -> Result<String> {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).context("secret Binance tidak valid")?;
    mac.update(query.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

fn parse_kline_row(row: &[serde_json::Value]) -> Option<KlinePoint> {
    if row.len() < 6 {
        return None;
    }

    Some(KlinePoint {
        open_time: row.first()?.as_i64()?,
        open: parse_value(&row[1]),
        high: parse_value(&row[2]),
        low: parse_value(&row[3]),
        close: parse_value(&row[4]),
        volume: parse_value(&row[5]),
    })
}

fn parse_value(value: &serde_json::Value) -> f64 {
    value.as_str().map(parse_number).unwrap_or_default()
}

fn parse_number(value: &str) -> f64 {
    value.parse::<f64>().unwrap_or_default()
}

fn unix_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn is_stablecoin(asset: &str) -> bool {
    matches!(asset, "USDT" | "FDUSD" | "USDC" | "BUSD" | "TUSD" | "DAI")
}

fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        return "***".into();
    }

    format!("{}...{}", &key[..4], &key[key.len().saturating_sub(4)..])
}

#[derive(Debug, Clone)]
struct Credentials {
    api_key: String,
    api_secret: String,
}

#[derive(Debug, Deserialize)]
struct RawTicker24hr {
    symbol: String,
    #[serde(rename = "lastPrice")]
    last_price: String,
    #[serde(rename = "priceChangePercent")]
    price_change_percent: String,
    #[serde(rename = "highPrice")]
    high_price: String,
    #[serde(rename = "lowPrice")]
    low_price: String,
    volume: String,
    #[serde(rename = "quoteVolume")]
    quote_volume: String,
}

#[derive(Debug, Deserialize)]
struct RawPriceTicker {
    price: String,
}

#[derive(Debug, Deserialize)]
struct RawAccount {
    balances: Vec<RawBalance>,
}

#[derive(Debug, Deserialize)]
struct RawBalance {
    asset: String,
    free: String,
    locked: String,
}
