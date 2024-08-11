use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default, sqlx::FromRow)]
pub struct ArbDiffSignal {
    pub id: i64,
    pub symbol: String,
    pub from_compare: String,
    pub to_compare: String,
    pub price_diff: Decimal,
    pub price_diff_rate: Decimal,
    pub binance_futures_price: Decimal,
    pub bybit_futures_price: Decimal,
    pub kucoin_futures_price: Decimal,
    pub created: String,
    pub updated: String,
    pub binance_updated: String,
    pub bybit_updated: String,
    pub kucoin_updated: String
}

