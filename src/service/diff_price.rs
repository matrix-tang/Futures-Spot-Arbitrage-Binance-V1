use crate::{model, sql};
use chrono::{NaiveDateTime, Utc};
use log::{debug, error, info};
use rust_decimal::Decimal;
use tokio::time::{sleep, timeout};
use std::ops::{Div, Mul, Sub};
use std::str::FromStr;
use std::time::Duration;
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
                        let updated = (Utc::now() + Duration::from_secs(2 * 60 * 60)).naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();
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

pub async fn get_diff_signal() {
    loop {
        let coin_prices = sql::get_all_arb_coin_prices().await.unwrap();
        for coin_price in coin_prices.clone() {
            let prices_data = vec![
                ("binance_futures_price", coin_price.binance_futures_price, coin_price.binance_updated),
                ("bybit_futures_price", coin_price.bybit_futures_price, coin_price.bybit_updated),
                ("kucoin_futures_price", coin_price.kucoin_futures_price, coin_price.kucoin_updated),
            ];

            for (i, (from_compare, from_price, from_updated)) in prices_data.iter().enumerate() {
                for (j, (to_compare, to_price, to_updated)) in prices_data.iter().enumerate() {
                    if i >= j || *from_price == Decimal::ZERO || *to_price == Decimal::ZERO {
                        continue;
                    }
                    // Check if the updated times are within the last 10 minutes
                    let now = Utc::now().naive_utc();
                    let ten_minutes_ago = now - Duration::from_secs(10 * 60);

                    let from_updated_ts = NaiveDateTime::parse_from_str(&from_updated, "%Y-%m-%d %H:%M:%S").unwrap();
                    let to_updated_ts = NaiveDateTime::parse_from_str(&to_updated, "%Y-%m-%d %H:%M:%S").unwrap();
                    if from_updated_ts < ten_minutes_ago || to_updated_ts < ten_minutes_ago {
                        continue;
                    }

                    let price_diff = to_price.sub(*from_price);
                    let price_diff_rate = price_diff.div(*to_price).mul(Decimal::from(100));
                    let threshold = Decimal::from_str("1.2").unwrap();
                    let updated = (Utc::now() + Duration::from_secs(2 * 60 * 60)).naive_utc().to_string();

                    if price_diff_rate > threshold {
                        // Check if the entry already exists in the diff_signal table
                        match sql::get_arb_diff_signal_by_symbol_from_and_to_compare(
                            &coin_price.symbol,
                            &from_compare.to_string(),
                            &to_compare.to_string(),
                        ).await {
                            Ok(existing_signal) => {
                                // Update the existing record
                                if let Err(e) = sql::update_arb_diff_signal_by_id(
                                    existing_signal.id,
                                    price_diff,
                                    price_diff_rate,
                                    *from_price,
                                    *to_price,
                                    from_updated.clone(),
                                    to_updated.clone()
                                ).await {
                                    error!("Failed to update arb_diff_signal: {:?}", e);
                                } else {
                                    info!("Successfully updated arb_diff_signal with id: {}", existing_signal.id);
                                }
                            },
                            Err(e) => {
                                // Log the error to understand why the record was not found
                                error!("Failed to find existing arb_diff_signal: {:?}", e);
                        
                                // Insert a new record
                                if let Err(e) = sql::insert_arb_diff_signal(
                                    model::ArbDiffSignal {
                                        id: 0,
                                        symbol: coin_price.symbol.clone(),
                                        from_compare: from_compare.to_string(),
                                        to_compare: to_compare.to_string(),
                                        price_diff,
                                        price_diff_rate,
                                        from_price: *from_price,
                                        to_price: *to_price,
                                        created: updated.clone(),
                                        from_updated: from_updated.clone(),
                                        to_updated: to_updated.clone()
                                    }
                                ).await {
                                    error!("Failed to insert new arb_diff_signal: {:?}", e);
                                } else {
                                    info!("Successfully inserted new arb_diff_signal for symbol: {}", coin_price.symbol);
                                }
                            }
                        }
                    }
                }
            }
        }
        sleep(Duration::from_secs(3)).await;
    }
}