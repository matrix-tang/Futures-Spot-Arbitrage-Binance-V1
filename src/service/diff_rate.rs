use crate::{model, service, sql};
use chrono::Local;
use log::{debug, error};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::ops::{Div, Sub};
use std::str::FromStr;

#[allow(unused_assignments)]
pub async fn set_binance_diff_rate() {
    let mut diff_rate_his_map: HashMap<i64, Decimal> = HashMap::new();
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        match sql::get_arb_diff_rate_list_by_diff_status(model::arb_diff_rate::DIFF_STATUS_RUN)
            .await
        {
            Ok(result) => {
                for diff_rate in result {
                    let mut from_price = Decimal::ZERO;
                    let mut to_price = Decimal::ZERO;
                    if let Ok(from_price_info) = service::get_binance_price(
                        diff_rate.from_market.clone(),
                        diff_rate.from_symbol.clone(),
                    )
                    .await
                    {
                        if let Ok(price) =
                            Decimal::from_str(from_price_info.ticker.current_close.unwrap().as_str())
                        {
                            from_price = price;
                        }
                    }

                    if let Ok(to_price_info) = service::get_binance_price(
                        diff_rate.to_market.clone(),
                        diff_rate.to_symbol.clone(),
                    )
                    .await
                    {
                        if let Ok(price) =
                            Decimal::from_str(to_price_info.ticker.current_close.unwrap().as_str())
                        {
                            to_price = price;
                        }
                    }

                    // 计算差价、比率
                    let mut diff = Decimal::ZERO;
                    let mut rate = Decimal::ZERO;
                    let mut info_rate = Decimal::ZERO;

                    if diff_rate.option_choose == "positive".to_string() {
                        diff = to_price.sub(from_price);
                        rate = diff.div(from_price);
                        rate.rescale(4 as u32);
                        info_rate = rate;
                        info_rate.rescale(3 as u32);
                    } else {
                        diff = from_price.sub(to_price);
                        rate = diff.div(to_price);
                        rate.rescale(4 as u32);
                        info_rate = rate;
                        info_rate.rescale(3 as u32);
                    }

                    debug!(
                        "option_choose: {:?}, from_symbol: {:?}, to_symbol: {:?}, from: {:?}, to: {:?}, diff: {:?}, rate: {:?}, info_rate: {:?}",
                        diff_rate.option_choose.clone(), diff_rate.from_symbol.clone(), diff_rate.to_symbol.clone(), from_price, to_price, diff, rate, info_rate
                    );

                    // 跳过差价相同记录
                    if let Some(map_rate) = diff_rate_his_map.get(&diff_rate.id) {
                        if map_rate.ne(&info_rate) {
                            if let Err(e) = sql::insert_arb_diff_rate_his(model::ArbDiffRateHis {
                                id: 0,
                                diff_rate_id: diff_rate.id,
                                diff_price: diff,
                                diff_rate: info_rate,
                                created: Some(Local::now().timestamp()),
                                updated: Some(Local::now().timestamp()),
                                bak: None,
                            })
                            .await
                            {
                                error!("{:?}", e);
                                // continue;
                            }
                            diff_rate_his_map.insert(diff_rate.id, info_rate);
                        }
                    } else {
                        if let Err(e) = sql::insert_arb_diff_rate_his(model::ArbDiffRateHis {
                            id: 0,
                            diff_rate_id: diff_rate.id,
                            diff_price: diff,
                            diff_rate: info_rate,
                            created: Some(Local::now().timestamp()),
                            updated: Some(Local::now().timestamp()),
                            bak: None,
                        })
                        .await
                        {
                            error!("{:?}", e);
                            // continue;
                        }
                        diff_rate_his_map.insert(diff_rate.id, info_rate);
                    }

                    // 设置info表
                    if let Ok(diff_rate_info) =
                        sql::get_arb_diff_rate_info_by_diff_rate_id(diff_rate.id).await
                    {
                        let _ = sql::update_arb_diff_rate_info_by_id(
                            diff_rate_info.id,
                            from_price,
                            to_price,
                            diff,
                            rate,
                        )
                        .await;
                    } else {
                        if let Err(e) = sql::insert_arb_diff_rate_info(model::ArbDiffRateInfo {
                            id: 0,
                            diff_rate_id: diff_rate.id,
                            platform: diff_rate.platform,
                            coin: diff_rate.coin,
                            option_choose: diff_rate.option_choose,
                            from_market: diff_rate.from_market,
                            from_symbol: diff_rate.from_symbol,
                            from_price,
                            to_market: diff_rate.to_market,
                            to_symbol: diff_rate.to_symbol,
                            to_price,
                            diff_price: diff,
                            diff_rate: rate,
                            created: Some(Local::now().timestamp()),
                            updated: Some(Local::now().timestamp()),
                            bak: None,
                        })
                        .await
                        {
                            error!("{:?}", e);
                            continue;
                        }
                    }
                }
            }
            Err(e) => {
                error!("{:?}", e);
            }
        }
    }
}
