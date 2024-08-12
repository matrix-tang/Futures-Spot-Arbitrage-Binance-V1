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
    pub from_price: Decimal,
    pub to_price: Decimal,
    pub created: String,
    pub updated: String,
    pub from_updated: String,
    pub to_updated: String,
    pub state: SignalState
}


#[derive(Debug, Clone, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "signal_state", rename_all = "UPPERCASE")]
pub enum SignalState {
    NotStarted,
    Open,
    Closed,
}

// Implement Default for SignalState
impl Default for SignalState {
    fn default() -> Self {
        SignalState::NotStarted
    }
}

