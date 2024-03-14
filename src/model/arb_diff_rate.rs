use serde::{Deserialize, Serialize};

pub const DIFF_STATUS_UN_RUN: i8 = 0;
pub const DIFF_STATUS_RUN: i8 = 1;

#[derive(Debug, Clone, Deserialize, Serialize, Default, sqlx::FromRow)]
pub struct ArbDiffRate {
    pub id: i64,
    pub platform: String,
    pub coin: String,
    pub option_choose: String,
    pub from_market: String,
    pub from_symbol: String,
    pub to_market: String,
    pub to_symbol: String,
    pub investment_currency: String,
    pub return_currency: String,
    pub diff_status: i8,
    pub created: Option<i64>,
    pub updated: Option<i64>,
    pub bak: Option<String>,
}
