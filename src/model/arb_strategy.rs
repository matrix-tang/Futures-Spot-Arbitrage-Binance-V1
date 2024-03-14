use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub const DOING_STATUS_UN_RUN: i8 = 0;
pub const DOING_STATUS_RUN: i8 = 1;
pub const DOING_STATUS_DONE: i8 = 2;

#[derive(Debug, Clone, Deserialize, Serialize, Default, sqlx::FromRow)]
pub struct ArbStrategy {
    pub id: i64,
    pub diff_rate_id: i64,
    pub user_id: i64,
    pub platform: String,
    pub option_choose: String,
    pub coin: String,
    pub from_market: String,
    pub from_symbol: String,
    pub from_price_truncate: i8,
    pub from_amt_truncate: i8,
    pub to_market: String,
    pub to_symbol: String,
    pub to_price_truncate: i8,
    pub to_amt_truncate: i8,
    pub from_to_desc: String,
    pub to_from_desc: String,
    pub option_open: Decimal,
    pub option_close: Decimal,
    pub option_amt: Decimal,
    pub contract_mul: i64,
    pub margin_mul: i64,
    pub fok_diff: Decimal,
    pub spot_fee: Decimal,
    pub futures_fee: Decimal,
    pub delivery_fee: Decimal,
    pub doing_status: i8,
    pub created: Option<i64>,
    pub updated: Option<i64>,
    pub bak: Option<String>,
}
