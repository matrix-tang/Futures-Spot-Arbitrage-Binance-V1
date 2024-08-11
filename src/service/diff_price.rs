use crate::{model, sql};
use chrono::{Duration, Utc};
use log::{debug, error};
use rust_decimal::Decimal;
use std::ops::{Div, Mul, Sub};
use std::str::FromStr;
use tokio::select;
use tokio::sync::mpsc::UnboundedReceiver;
use crate::binance::ws_model::{Market, Platform};
use crate::service::PriceStream;

// src/service/diff_price.rs

pub async fn set_price_data(mut price_rx: UnboundedReceiver<PriceStream>) {
    loop {
        select! {
            event = price_rx.recv() => {
                if let Some(stream) = event {
                    for ticker in stream.tickers {
                        if ticker.current_close.is_none() {
                            continue;
                        }
                        let symbol = ticker.symbol.clone();
                        let price = Decimal::from_str(&ticker.current_close.unwrap()).unwrap();
                        let platform = stream.platform.clone();
                        let market = stream.market.clone();
                        let updated = (Utc::now() + Duration::try_hours(2).unwrap()).naive_utc().to_string();
                        debug!("got data for symbol: {symbol}, price: {price}, platform: {platform}, market: {market}");
                        
                        let (price_field, date_field) = match platform {
                            Platform::Binance => match market {
                                Market::Spot => ("binance_spot_price", "binance_updated"),
                                Market::Futures => ("binance_futures_price", "binance_updated"),
                            },
                            Platform::Bybit => match market {
                                Market::Spot => ("bybit_spot_price", "bybit_updated"),
                                Market::Futures => ("bybit_futures_price", "bybit_updated"),
                            },
                            Platform::Kucoin => match market {
                                Market::Spot => ("kucoin_spot_price", "kucoin_updated"),
                                Market::Futures => ("kucoin_futures_price", "kucoin_updated"),
                            }
                        };
                        // Check if the record exists
                        if let Ok(existing_record) = sql::get_arb_coin_price_by_symbol(&symbol).await {
                            // Update the existing record
                            if let Err(e) = sql::update_arb_coin_price_by_id(
                                existing_record.id,
                                price_field,
                                price,
                                date_field,
                                updated.clone(),
                            ).await {
                                error!("{:?}", e);
                            }
                        } else {
                            // Insert a new record
                            let mut new_record = model::ArbCoinPrice {
                                id: 0,
                                platform: platform.as_str().parse().unwrap(),
                                symbol,
                                binance_spot_price: Decimal::ZERO,
                                binance_futures_price: Decimal::ZERO,
                                bybit_futures_price: Decimal::ZERO,
                                kucoin_futures_price: Decimal::ZERO,
                                created: updated.clone(),
                                bak: None,
                                bybit_updated: "Not Available".to_string(),
                                kucoin_updated: "Not Available".to_string(),
                                binance_updated: "Not Available".to_string(),
                            };
                            match platform {
                                Platform::Binance => {
                                    new_record.binance_futures_price = price;
                                    new_record.binance_updated = updated;
                                }
                                Platform::Bybit => {
                                    new_record.bybit_futures_price = price;
                                    new_record.bybit_updated = updated;
                                }
                                Platform::Kucoin => {
                                    new_record.kucoin_futures_price = price;
                                    new_record.kucoin_updated = updated;
                                }
                            }
                            if let Err(e) = sql::insert_arb_coin_price(new_record).await {
                                error!("{:?}", e);
                            }
                        }
                    }
                }
            },
        }
    }
}
//
// pub async fn set_price_data(mut price_rx: UnboundedReceiver<PriceStream>) {
//     let up = (Utc::now() + Duration::hours(2)).naive_utc().to_string();
//     loop {
//         select! {
//             event = price_rx.recv() => {
//                 if let Some(stream) = event {
//                     for ticker in stream.tickers {
//                         if ticker.current_close.is_none() {
//                             continue;
//                         }
//                         let symbol = ticker.symbol.clone();
//                         let price = Decimal::from_str(&ticker.current_close.unwrap()).unwrap();
//                         let market = stream.market.clone();
//                         let platform = stream.platform.clone();
//                         let updated = (Utc::now() + Duration::hours(2)).naive_utc().to_string();
//                         // Check if the record exists
//                         if let Ok(existing_record) = sql::get_arb_coin_price_by_symbol(&symbol).await {
//
//                             let (spot_price, future_price) = match market {
//                                 Market::Spot => (price, existing_record.future_price.clone()),
//                                 Market::Futures => (existing_record.spot_price.clone(), price),
//                                 _ => (existing_record.spot_price.clone(), existing_record.future_price.clone()), // Default case, should not happen
//                             };
//                             // Update the existing record
//                             if let Err(e) = sql::update_arb_coin_price_by_id(
//                                 existing_record.id,
//                                 spot_price,
//                                 future_price,
//                                 updated.clone(),
//                             ).await {
//                                 error!("{:?}", e);
//                             }
//                         } else {
//                             let (spot_price, future_price) = match market {
//                                 Market::Spot => (price, Decimal::ZERO),
//                                 Market::Futures => (Decimal::ZERO, price),
//                                 _ => (Decimal::ZERO, Decimal::ZERO), // Default case, should not happen
//                             };
//                             // Insert a new record
//                             if let Err(e) = sql::insert_arb_coin_price(
//                                 model::ArbCoinPrice {
//                                     id: 0,
//                                     platform: platform.as_str().parse().unwrap(),
//                                     symbol,
//                                     spot_price,
//                                     future_price,
//                                     created: updated.clone(),
//                                     updated,
//                                     bak: None
//                                 }
//                             ).await {
//                                 error!("{:?}", e);
//                             }
//                         }
//                     }
//                 }
//             },
//         }
//     }
// }

#[allow(unused_assignments)]
pub async fn get_diff_signal() {
    loop {
        // Fetch all rows from the arb_coin_price table
        let coin_prices = sql::get_all_arb_coin_prices().await.unwrap();

        for coin_price in coin_prices {
            let prices = vec![
                ("binance_futures_price", coin_price.binance_futures_price),
                ("bybit_futures_price", coin_price.bybit_futures_price),
                ("kucoin_futures_price", coin_price.kucoin_futures_price),
            ];

            for (i, (from_compare, from_price)) in prices.iter().enumerate() {
                for (j, (to_compare, to_price)) in prices.iter().enumerate() {
                    if i >= j || *from_price == Decimal::ZERO || *to_price == Decimal::ZERO {
                        continue;
                    }

                    let price_diff = to_price.sub(*from_price);
                    let price_diff_rate = price_diff.div(*to_price).mul(Decimal::from(100));
                    let threshold = Decimal::from_str("1.2").unwrap();
                    let updated = (Utc::now() + Duration::try_hours(2).unwrap()).naive_utc().to_string();

                    if price_diff_rate > threshold {
                        // Check if the entry already exists in the diff_signal table
                        if let Ok(existing_signal) = sql::get_arb_diff_signal_by_symbol_from_and_to_compare(
                            &coin_price.symbol,
                            &from_compare.to_string(),
                            &to_compare.to_string(),
                        ).await {
                            // Update the existing record
                            if let Err(e) = sql::update_arb_diff_signal_by_id(
                                existing_signal.id,
                                price_diff,
                                price_diff_rate,
                                coin_price.binance_futures_price,
                                coin_price.bybit_futures_price,
                                coin_price.kucoin_futures_price,
                                updated.clone(),
                            ).await {
                                error!("{:?}", e);
                            }
                        } else {
                            // Insert a new record
                            if let Err(e) = sql::insert_arb_diff_signal(
                                model::ArbDiffSignal {
                                    id: 0,
                                    symbol: coin_price.symbol.clone(),
                                    from_compare: from_compare.to_string(),
                                    to_compare: to_compare.to_string(),
                                    price_diff,
                                    price_diff_rate,
                                    binance_futures_price: coin_price.binance_futures_price,
                                    bybit_futures_price: coin_price.bybit_futures_price,
                                    kucoin_futures_price: coin_price.kucoin_futures_price,
                                    created: updated.clone(),
                                    updated,
                                    binance_updated: coin_price.binance_updated.clone(),
                                    bybit_updated: coin_price.bybit_updated.clone(),
                                    kucoin_updated: coin_price.kucoin_updated.clone(),
                                }
                            ).await {
                                error!("{:?}", e);
                            }
                        }
                    }
                }
            }
        }
    }
}