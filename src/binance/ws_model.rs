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
#[serde(rename_all = "camelCase")]
pub struct TickerData {
    symbol: String,
    tick_direction: Option<String>,
    price_24h_pcnt: Option<String>,
    last_price: Option<String>,
    turnover_24h: Option<String>,
    volume_24h: Option<String>,
    bid1_price: Option<String>,
    bid1_size: Option<String>,
    ask1_price: Option<String>,
    ask1_size: Option<String>,
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
            current_close: bybit_event.data.last_price,
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
            current_close: kucoin_event.data.mark_price.map(|value| value.to_string()),
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
pub struct SubscribeResponse {
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
#[serde(rename_all = "camelCase")]
pub struct KucoinTickerData {
    symbol: String,
    mark_price: Option<f64>,
    index_price: Option<f64>,
    granularity: Option<u64>,
    funding_rate: Option<f64>,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KucoinTickerMessage {
    topic: String,
    r#type: String,  // `type` is a reserved keyword, so use `r#type`
    data: KucoinTickerData,
    subject: String,
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
