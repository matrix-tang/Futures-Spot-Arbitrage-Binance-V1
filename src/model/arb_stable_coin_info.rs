use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub const IS_OK_UN: i8 = 0;
pub const IS_OK_COMPLETED: i8 = 1;
pub const IS_OK_EXPIRED: i8 = 2;

#[derive(Debug, Clone, Deserialize, Serialize, Default, sqlx::FromRow)]
pub struct ArbStableCoinInfo {
    pub id: i64,
    pub stable_coin_id: i64,
    pub user_id: i64,
    pub platform: String,
    pub coin: String,
    pub market: String,
    pub symbol: String,
    pub option_type: String,
    pub price: Decimal,
    pub amount: Decimal,
    pub order_id: String,
    pub is_ok: i8,
    pub created: Option<i64>,
    pub updated: Option<i64>,
    pub bak: Option<String>,
}
