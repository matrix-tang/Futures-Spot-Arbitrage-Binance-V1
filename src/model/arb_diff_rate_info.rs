use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default, sqlx::FromRow)]
pub struct ArbDiffRateInfo {
    pub id: i64,
    pub diff_rate_id: i64,
    pub platform: String,
    pub coin: String,
    pub option_choose: String,
    pub from_market: String,
    pub from_symbol: String,
    pub from_price: Decimal,
    pub to_market: String,
    pub to_symbol: String,
    pub to_price: Decimal,
    pub diff_price: Decimal,
    pub diff_rate: Decimal,
    pub created: Option<i64>,
    pub updated: Option<i64>,
    pub bak: Option<String>,
}
