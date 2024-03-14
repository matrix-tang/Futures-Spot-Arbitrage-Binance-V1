use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub const OPTION_STATUS_UN_DONE: i8 = 0;
pub const OPTION_STATUS_DONE: i8 = 1;

#[derive(Debug, Clone, Deserialize, Serialize, Default, sqlx::FromRow)]
pub struct ArbStrategyEx {
    pub id: i64,
    pub user_id: i64,
    pub platform: String,
    pub option_choose: String,
    pub arb_strategy_id: i64,
    pub coin: String,
    pub market: String,
    pub symbol: String,
    pub option_type: String,
    pub option_status: i8,
    pub option_amount: Decimal,
    pub option_executed_amt: Decimal,
    pub current_order_id: String,
    pub created: Option<i64>,
    pub updated: Option<i64>,
    pub bak: Option<String>,
}
