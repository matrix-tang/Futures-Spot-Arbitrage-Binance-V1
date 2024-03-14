use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default, sqlx::FromRow)]
pub struct ArbDiffRateHis {
    pub id: i64,
    pub diff_rate_id: i64,
    pub diff_price: Decimal,
    pub diff_rate: Decimal,
    pub created: Option<i64>,
    pub updated: Option<i64>,
    pub bak: Option<String>,
}
