use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WebSocketEvent {
    TickerDelta(Box<BybitTickerEvent>),
    MiniTicker(Box<Vec<BinanceTickerEvent>>),
    Subscribe(Box<SubscribeResponse>),
    KucoinTicker(Box<KucoinTickerMessage>),
    Welcome(Box<WelcomeMessage>)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BybitTickerEvent {
    topic: String,
    r#type: String,
    data: TickerData,
    cs: Option<u64>,
    ts: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TickerData {
    symbol: String,
    tickDirection: Option<String>,
    price24hPcnt: Option<String>,
    lastPrice: Option<String>,
    turnover24h: Option<String>,
    volume24h: Option<String>,
    bid1Price: Option<String>,
    bid1Size: Option<String>,
    ask1Price: Option<String>,
    ask1Size: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceTickerEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub current_close: Option<String>,
    #[serde(rename = "o")]
    pub open: Option<String>,
    #[serde(rename = "h")]
    pub high: Option<String>,
    #[serde(rename = "l")]
    pub low: Option<String>,
    #[serde(rename = "v")]
    pub volume: Option<String>,
    #[serde(rename = "q")]
    pub quote_volume: Option<String>,
}

impl From<Box<BybitTickerEvent>> for BinanceTickerEvent {
    fn from(bybit_event: Box<BybitTickerEvent>) -> Self {
        BinanceTickerEvent {
            event_type: "".to_string(),
            event_time: chrono::Local::now().timestamp_millis() as u64,
            symbol: bybit_event.data.symbol,
            current_close: bybit_event.data.lastPrice,
            open: None,
            high: None,
            low: None,
            volume: None,
            quote_volume: None,
        }
    }
}

impl From<Box<KucoinTickerMessage>> for BinanceTickerEvent {
    fn from(kucoin_event: Box<KucoinTickerMessage>) -> Self {
        BinanceTickerEvent {
            event_type: "".to_string(),
            event_time: chrono::Local::now().timestamp_millis() as u64,
            symbol: {
                let mut symbol = kucoin_event.data.symbol.clone();
                if !symbol.is_empty() {
                    symbol.pop();
                }
                symbol
            },
            current_close: kucoin_event.data.markPrice.map(|value| value.to_string()),
            open: None,
            high: None,
            low: None,
            volume: None,
            quote_volume: None,
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let platform_str = match self {
            Platform::Binance => "Binance",
            Platform::Bybit => "Bybit",
            Platform::Kucoin => "Kucoin",
        };
        write!(f, "{}", platform_str)
    }
}

impl fmt::Display for Market {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let market_str = match self {
            Market::Spot => "Spot",
            Market::Futures => "Futures",
        };
        write!(f, "{}", market_str)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct SubscribeResponse {
    success: bool,
    ret_msg: String,
    conn_id: String,
    req_id: String,
    op: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Platform {
    Binance,
    Bybit,
    Kucoin,
}

impl Platform {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Platform::Binance => "Binance",
            Platform::Bybit => "Bybit",
            Platform::Kucoin => "Kucoin",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Market {
    Spot,
    Futures,
}

impl Market {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Market::Spot => "spot",
            Market::Futures => "futures",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KucoinTickerData {
    symbol: String,
    markPrice: Option<f64>,
    indexPrice: Option<f64>,
    granularity: Option<u64>,
    fundingRate: Option<f64>,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KucoinTickerMessage {
    topic: String,
    r#type: String,  // `type` is a reserved keyword, so use `r#type`
    data: KucoinTickerData,
    subject: String,
}

fn convert_ticker_string(ticker: &String) -> String {
    // Split by colon and take the second part
    let parts: Vec<&str> = ticker.split(':').collect();
    if parts.len() < 2 {
        return String::new();
    }
    let symbol = parts[1];

    // Split by hyphen and concatenate
    let currencies: Vec<&str> = symbol.split('-').collect();
    if currencies.len() < 2 {
        return String::new();
    }
    format!("{}{}", currencies[0], currencies[1])
}
#[derive(Debug, Deserialize, Serialize)]
pub struct WelcomeMessage {
    id: String,
    r#type: String,  // `type` is a reserved keyword, so use `r#type`
}
#[derive(Deserialize)]
pub struct TokenResponse {
    pub data: TokenData,
}

#[derive(Deserialize)]
pub struct TokenData {
    pub token: String,
}
