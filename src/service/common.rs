use crate::model;
use std::collections::HashMap;

// 生成 arb_strategy_ex 描述
#[derive(Debug, Clone)]
pub struct ExDesc {
    pub market: String,
    pub symbol: String,
    pub option_type: String,
}

pub fn new_positive_desc(s: model::ArbStrategy) -> anyhow::Result<HashMap<i32, ExDesc>> {
    let mut ex_desc_map = HashMap::new();
    ex_desc_map.insert(
        0,
        ExDesc {
            market: s.from_market.clone(),
            symbol: s.from_symbol.clone(),
            option_type: "spot_buy".to_string(),
        },
    );

    ex_desc_map.insert(
        1,
        ExDesc {
            market: "transfer".to_string(),
            symbol: s.coin.clone(),
            option_type: "transfer_spot_to_delivery".to_string(),
        },
    );

    ex_desc_map.insert(
        2,
        ExDesc {
            market: s.to_market.clone(),
            symbol: s.to_symbol.clone(),
            option_type: "delivery_sell".to_string(),
        },
    );

    ex_desc_map.insert(
        3,
        ExDesc {
            market: s.to_market,
            symbol: s.to_symbol,
            option_type: "delivery_buy".to_string(),
        },
    );

    ex_desc_map.insert(
        4,
        ExDesc {
            market: "transfer".to_string(),
            symbol: s.coin,
            option_type: "transfer_delivery_to_spot".to_string(),
        },
    );

    ex_desc_map.insert(
        5,
        ExDesc {
            market: s.from_market,
            symbol: s.from_symbol,
            option_type: "spot_sell".to_string(),
        },
    );

    Ok(ex_desc_map)
}

pub fn new_reverse_usdm_desc(s: model::ArbStrategy) -> anyhow::Result<HashMap<i32, ExDesc>> {
    let mut ex_desc_map = HashMap::new();
    ex_desc_map.insert(
        0,
        ExDesc {
            market: s.from_market.clone(),
            symbol: s.from_symbol.clone(),
            option_type: "futures_buy".to_string(),
        },
    );

    ex_desc_map.insert(
        1,
        ExDesc {
            market: s.to_market.clone(),
            symbol: s.to_symbol.clone(),
            option_type: "futures_sell".to_string(),
        },
    );

    ex_desc_map.insert(
        2,
        ExDesc {
            market: s.to_market,
            symbol: s.to_symbol,
            option_type: "futures_buy".to_string(),
        },
    );

    ex_desc_map.insert(
        3,
        ExDesc {
            market: s.from_market,
            symbol: s.from_symbol,
            option_type: "futures_sell".to_string(),
        },
    );

    Ok(ex_desc_map)
}

pub fn new_reverse_coinm_desc(s: model::ArbStrategy) -> anyhow::Result<HashMap<i32, ExDesc>> {
    let mut ex_desc_map = HashMap::new();
    ex_desc_map.insert(
        0,
        ExDesc {
            market: s.from_market.clone(),
            symbol: s.from_symbol.clone(),
            option_type: "delivery_buy".to_string(),
        },
    );

    ex_desc_map.insert(
        1,
        ExDesc {
            market: s.to_market.clone(),
            symbol: s.to_symbol.clone(),
            option_type: "delivery_sell".to_string(),
        },
    );

    ex_desc_map.insert(
        2,
        ExDesc {
            market: s.to_market,
            symbol: s.to_symbol,
            option_type: "delivery_buy".to_string(),
        },
    );

    ex_desc_map.insert(
        3,
        ExDesc {
            market: s.from_market,
            symbol: s.from_symbol,
            option_type: "delivery_sell".to_string(),
        },
    );

    Ok(ex_desc_map)
}
