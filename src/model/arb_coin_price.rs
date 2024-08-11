use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub const DIFF_STATUS_UN_RUN: i8 = 0;
pub const DIFF_STATUS_RUN: i8 = 1;

#[derive(Debug, Clone, Deserialize, Serialize, Default, sqlx::FromRow)]
pub struct ArbCoinPrice {
    pub id: i64,
    pub platform: String,
    pub symbol: String,
    pub binance_spot_price: Decimal,
    pub binance_futures_price: Decimal,
    pub bybit_futures_price: Decimal,
    pub kucoin_futures_price: Decimal,
    pub created: String,
    pub bybit_updated: String,
    pub kucoin_updated: String,
    pub binance_updated: String,
}