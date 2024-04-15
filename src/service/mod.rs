pub mod binance_strategy;
mod common;
pub mod diff_rate;
pub mod price;
pub mod stable_coin_hedging;

pub use binance_strategy::event_start;
pub use binance_strategy::inspect_strategy;
pub use binance_strategy::range_new_strategy;
pub use diff_rate::set_binance_diff_rate;
pub use price::get_binance_price;
pub use price::set_binance_price;
pub use stable_coin_hedging::event_stable_coin_start;
pub use stable_coin_hedging::inspect_stable_coin;

use crate::binance::websockets::*;
use crate::binance::ws_model::*;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceStream {
    pub tickers: Vec<MiniDayTickerEvent>,
    pub market: String,
    pub local_time: i64,
}

#[allow(dead_code)]
#[allow(irrefutable_let_patterns)]
pub async fn spot_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    let keep_running = AtomicBool::new(true);
    let all_ticker = all_mini_ticker_stream();

    let mut web_socket: WebSockets<'_, Vec<WebsocketEvent>> =
        WebSockets::new(|events: Vec<WebsocketEvent>| {
            let mut tickers = Vec::new();
            for tick_events in events {
                if let WebsocketEvent::DayMiniTicker(tick_event) = tick_events {
                    tickers.push(*tick_event)
                }
            }

            let price_stream = PriceStream {
                tickers: tickers,
                market: "spot".to_string(),
                local_time: chrono::Local::now().timestamp_millis(),
            };
            match price_tx.send(price_stream) {
                Ok(_) => {}
                Err(_e) => {
                    keep_running.store(false, Ordering::Relaxed);
                }
            }

            // Break the event loop
            // keep_running.store(false, Ordering::Relaxed);
            Ok(())
        });

    web_socket.connect(all_ticker).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        error!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    info!("spot websocket disconnected");
}

#[allow(dead_code)]
#[allow(irrefutable_let_patterns)]
pub async fn futures_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    let keep_running = AtomicBool::new(true);
    let all_ticker = all_mini_ticker_stream();

    let mut web_socket: WebSockets<'_, Vec<WebsocketEvent>> =
        WebSockets::new(|events: Vec<WebsocketEvent>| {
            let mut tickers = Vec::new();
            for tick_events in events {
                if let WebsocketEvent::DayMiniTicker(tick_event) = tick_events {
                    tickers.push(*tick_event)
                }
            }

            let price_stream = PriceStream {
                tickers: tickers,
                market: "futures".to_string(),
                local_time: chrono::Local::now().timestamp_millis(),
            };
            match price_tx.send(price_stream) {
                Ok(_) => {}
                Err(_e) => {
                    keep_running.store(false, Ordering::Relaxed);
                }
            }

            // Break the event loop
            // keep_running.store(false, Ordering::Relaxed);
            Ok(())
        });

    web_socket.connect_futures(all_ticker).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        error!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    info!("futures websocket disconnected");
}

#[allow(dead_code)]
#[allow(irrefutable_let_patterns)]
pub async fn delivery_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    let keep_running = AtomicBool::new(true);
    let all_ticker = all_mini_ticker_stream();

    let mut web_socket: WebSockets<'_, Vec<WebsocketEvent>> =
        WebSockets::new(|events: Vec<WebsocketEvent>| {
            let mut tickers = Vec::new();
            for tick_events in events {
                if let WebsocketEvent::DayMiniTicker(tick_event) = tick_events {
                    tickers.push(*tick_event)
                }
            }

            let price_stream = PriceStream {
                tickers: tickers,
                market: "delivery".to_string(),
                local_time: chrono::Local::now().timestamp_millis(),
            };
            match price_tx.send(price_stream) {
                Ok(_) => {}
                Err(_e) => {
                    keep_running.store(false, Ordering::Relaxed);
                }
            }

            // Break the event loop
            // keep_running.store(false, Ordering::Relaxed);
            Ok(())
        });

    web_socket.connect_delivery(all_ticker).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        error!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    info!("delivery websocket disconnected");
}
