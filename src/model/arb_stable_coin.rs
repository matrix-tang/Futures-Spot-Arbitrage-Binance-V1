use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub const DOING_STATUS_UN_RUN: i8 = 0;
pub const DOING_STATUS_RUN: i8 = 1;
pub const DOING_STATUS_DONE: i8 = 2;

#[derive(Debug, Clone, Deserialize, Serialize, Default, sqlx::FromRow)]
pub struct ArbStableCoin {
    pub id: i64,
    pub user_id: i64,
    pub platform: String,
    pub coin: String,
    pub market: String,
    pub symbol: String,
    pub price_truncate: i8,
    pub amt_truncate: i8,
    pub strategy: String,
    pub option_open: Decimal,
    pub option_close: Decimal,
    pub option_amt: Decimal,
    pub fok_diff: Decimal,
    pub doing_status: i8,
    pub created: Option<i64>,
    pub updated: Option<i64>,
    pub bak: Option<String>,
}
