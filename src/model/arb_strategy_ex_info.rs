use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub const IS_OK_UN_DONE: i8 = 0;
pub const IS_OK_DONE: i8 = 1;
pub const IS_OK_EXPIRED: i8 = 2;

#[derive(Debug, Clone, Deserialize, Serialize, Default, sqlx::FromRow)]
pub struct ArbStrategyExInfo {
    pub id: i64,
    pub user_id: i64,
    pub platform: String,
    pub option_choose: String,
    pub arb_strategy_id: i64,
    pub arb_strategy_ex_id: i64,
    pub coin: String,
    pub market: String,
    pub symbol: String,
    pub option_type: String,
    pub price: Decimal,
    pub amount: Decimal,
    pub executed_amt: Decimal,
    pub order_id: String,
    pub is_ok: i8,
    pub created: Option<i64>,
    pub updated: Option<i64>,
    pub bak: Option<String>,
}
