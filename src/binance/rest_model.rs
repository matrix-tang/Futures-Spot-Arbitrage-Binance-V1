#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FuturesTransaction {
    pub client_order_id: String,
    pub cum_qty: Option<String>,
    pub cum_quote: Option<String>,
    #[serde(with = "string_or_float")]
    pub cum_base: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub avg_price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub reduce_only: bool,
    pub side: OrderSide,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub orig_type: String,
    pub activate_price: Option<String>,
    pub price_rate: Option<f64>,
    pub update_time: u64,
    pub working_type: String,
    pub price_protect: bool,
    pub price_match: Option<String>,
    pub self_trade_prevention_mode: Option<String>,
    pub good_till_date: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionId {
    pub tran_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UniversalTransferType {
    /// Spot account transfer to USDⓈ-M Futures account
    MainUmfuture,
    /// Spot account transfer to COIN-M Futures account
    MainCmfuture,
    /// Spot account transfer to Margin (cross) account
    MainMargin,
    /// USDⓈ-M Futures account transfer to Spot account
    UmfutureMain,
    /// USDⓈ-M Futures account transfer to Margin (cross) account
    UmFutureMargin,
    /// COIN-M Futures account transfer to Spot account
    CmfutureMain,
    /// COIN-M Futures account transfer to Margin(cross) account
    CmfutureMargin,
    /// Margin（cross）account transfer to Spot account
    MarginMain,
    /// Margin (cross) account transfer to USDⓈ-M Futures
    MarginUmfuture,
    /// Margin (cross) account transfer to COIN-M Futures
    MarginCmfuture,
    /// Isolated margin account transfer to Margin (cross) account
    IsolatedmarginMargin,
    /// Margin (cross) account transfer to Isolated margin account
    MarginIsolatedmargin,
    /// Isolated margin account transfer to Isolated margin account
    IsolatedmarginIsolatedmargin,
    /// Spot account transfer to Funding account
    MainFunding,
    /// Funding account transfer to Spot account
    FundingMain,
    /// Funding account transfer to UMFUTURE account
    FundingUmfuture,
    /// UMFUTURE account transfer to Funding account
    UmfutureFunding,
    /// MARGIN account transfer to Funding account
    MarginFunding,
    /// Funding account transfer to Margin account
    FundingMargin,
    /// Funding account transfer to CMFUTURE account
    FundingCmfuture,
    /// CMFUTURE account transfer to Funding account
    CmfutureFunding,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransfer {
    pub asset: String,
    pub amount: f64,
    pub from_symbol: Option<String>,
    pub to_symbol: Option<String>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub transfer_type: UniversalTransferType,
}

/// How long will an order stay alive
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum TimeInForce {
    /// Good Till Canceled
    GTC,
    /// Immediate Or Cancel
    IOC,
    /// Fill or Kill
    FOK,
    /// Good till expired
    GTX,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderResponse {
    Ack,
    Result,
    Full,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
    #[serde(other)]
    Other,
}

/// By default, use market orders
impl Default for OrderType {
    fn default() -> Self {
        Self::Market
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// By default, buy
impl Default for OrderSide {
    fn default() -> Self {
        Self::Buy
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
    pub transact_time: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub cummulative_quote_qty: f64,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: OrderSide,
    pub fills: Vec<Fill>,
}

/// Status of an order, this can typically change over time
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// The order has been accepted by the engine.
    New,
    /// A part of the order has been filled.
    PartiallyFilled,
    /// The order has been completely filled.
    Filled,
    /// The order has been canceled by the user.
    Canceled,
    /// Currently unused
    PendingCancel,
    /// The order was not accepted by the engine and not processed.
    Rejected,
    /// The order was canceled according to the order type's rules (e.g. LIMIT FOK orders with no fill, LIMIT IOC or MARKET orders that partially fill) or by the exchange, (e.g. orders canceled during liquidation, orders canceled during maintenance)
    Expired,
    /// The order was canceled by the exchange due to STP trigger. (e.g. an order with EXPIRE_TAKER will match with existing orders on the book with the same account or same tradeGroupId)
    ExpiredInMatch,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(with = "string_or_float")]
    pub commission: f64,
    pub commission_asset: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i32,
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub cummulative_quote_qty: f64,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: OrderSide,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    #[serde(with = "string_or_float")]
    pub iceberg_qty: f64,
    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,
    #[serde(with = "string_or_float")]
    pub orig_quote_order_qty: f64,
}

pub mod string_or_float {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}

#[allow(dead_code)]
pub(crate) mod string_or_float_opt {
    use std::fmt;

    use serde::{Deserializer, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        match value {
            Some(v) => crate::binance::rest_model::string_or_float::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Some(
            crate::binance::rest_model::string_or_float::deserialize(deserializer)?,
        ))
    }
}

pub mod string_or_u64 {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrU64 {
            String(String),
            U64(u64),
        }

        match StringOrU64::deserialize(deserializer)? {
            StringOrU64::String(s) => s.parse().map_err(de::Error::custom),
            StringOrU64::U64(i) => Ok(i),
        }
    }
}

pub mod string_or_u64_opt {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        match value {
            Some(v) => crate::binance::rest_model::string_or_u64::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrU64 {
            String(String),
            U64(u64),
        }

        match StringOrU64::deserialize(deserializer)? {
            StringOrU64::String(s) => s.parse().map_err(de::Error::custom).map(Some),
            StringOrU64::U64(i) => Ok(Some(i)),
        }
    }
}

#[allow(dead_code)]
pub(crate) mod string_or_bool {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Bool(bool),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Bool(i) => Ok(i),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum KlineSummaries {
    AllKlineSummaries(Vec<KlineSummary>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KlineSummary {
    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: i64,
    pub quote_asset_volume: f64,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: f64,
    pub taker_buy_quote_asset_volume: f64,
}
