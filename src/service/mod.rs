pub mod binance_strategy;
mod common;
pub mod diff_rate;
pub mod price;
pub mod stable_coin_hedging;
mod diff_price;

pub use diff_price::set_price_data;
pub use diff_price::get_diff_signal;
pub use binance_strategy::event_start;
pub use binance_strategy::inspect_strategy;
pub use binance_strategy::range_new_strategy;
pub use diff_rate::set_binance_diff_rate;
pub use price::get_binance_price;
pub use price::set_binance_price;
pub use stable_coin_hedging::event_stable_coin_start;
pub use stable_coin_hedging::inspect_stable_coin;

use crate::binance::errors::Error;
use crate::binance::websockets::*;
use crate::binance::ws_model::*;
use crate::conf::ws_message::BYBIT_MESSAGE;
use crate::conf::ws_message::KUCOIN_MESSAGE;
use crate::conf::C;
use log::{error, info};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceStream {
    pub tickers: Vec<BinanceTickerEvent>,
    pub market: Market,
    pub platform: Platform,
    pub local_time: i64,
}

async fn get_token() -> Result<String, reqwest::Error> {
    let url = "https://api-futures.kucoin.com/api/v1/bullet-public";
    let client = Client::new();
    let response = client.post(url).send().await?;
    let data: TokenResponse = response.json().await?;
    Ok(data.data.token)
}


pub async fn handle_websocket(
    platform: Platform,
    market: Market,
    price_tx: UnboundedSender<PriceStream>,
    keep_running: &AtomicBool,
) {
    loop {
        let mut web_socket: WebSockets<'_, WebSocketEvent> = WebSockets::new(|event: WebSocketEvent| {
            let mut tickers = Vec::new();
            match platform {
                Platform::Binance => {
                    if let WebSocketEvent::MiniTicker(tick_events) = event {
                        tickers.extend(*tick_events);
                    }
                }
                Platform::Bybit => {
                    if let WebSocketEvent::TickerDelta(bybit_event) = event {
                        tickers.push(bybit_event.into());
                    }
                }
                Platform::Kucoin => {
                    if let WebSocketEvent::KucoinTicker(ckucoin_event) = event {
                        tickers.push(ckucoin_event.into());
                    }
                }
            }
            let price_stream = PriceStream {
                tickers,
                platform: platform.clone(),
                market: market.clone(),
                local_time: chrono::Local::now().timestamp_millis(),
            };

            if price_tx.send(price_stream).is_err() {
                keep_running.store(false, Ordering::Relaxed);
            }

            Ok(())
        });
        match platform {
            Platform::Binance => {
                let url = match market {
                    Market::Spot => {
                        Url::parse(&C.binance_api_config.ws_spot_url).unwrap()
                    }
                    Market::Futures => {
                        Url::parse(&C.binance_api_config.ws_futures_url).unwrap()
                    }   
                };
                web_socket.connect(url).await.unwrap();
            }
            Platform::Bybit => {
                let (url, message) = match market {
                    Market::Spot => {
                        (Url::parse(&C.bybit_api_config.ws_futures_url).unwrap(), Message::Text(BYBIT_MESSAGE.to_string()))
                    }
                    Market::Futures => {
                        (Url::parse(&C.bybit_api_config.ws_futures_url).unwrap(), Message::Text(BYBIT_MESSAGE.to_string()))
                    } 
                };
                web_socket.connect(url).await.unwrap();
                web_socket.send_message(message).await.unwrap();
            }
            Platform::Kucoin => {
                let (url, message) = match market {
                    Market::Spot => {
                        let token = get_token().await.unwrap();
                        let url =  &C.kucoin_api_config.ws_futures_url.replace("{token}", &token);
                        (Url::parse(url).unwrap(), Message::Text(BYBIT_MESSAGE.to_string()))
                    }
                    Market::Futures => {
                        let token = get_token().await.unwrap();
                        let url =  &C.kucoin_api_config.ws_futures_url.replace("{token}", &token);
                        (Url::parse(url).unwrap(), Message::Text(KUCOIN_MESSAGE.to_string()))
                    } 
                };
                web_socket.connect(url).await.unwrap();
                web_socket.send_message(message).await.unwrap();
            }
            
        }
        info!("websocket connected for platform: {platform}, market: {market}");
        if let Err(e) = web_socket.event_loop(&keep_running).await {
            error!("platform: {platform}, market: {market}, Error: {e}");
            continue
        }
        web_socket.disconnect().await.unwrap();
        info!("websocket disconnected");
    }

}

#[allow(dead_code)]
#[allow(irrefutable_let_patterns)]
pub async fn binance_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    let keep_running = AtomicBool::new(true);
    tokio::join!(
        handle_websocket(Platform::Binance, Market::Futures, price_tx, &keep_running)
    );
}

#[allow(dead_code)]
#[allow(irrefutable_let_patterns)]
pub async fn bybit_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    let keep_running = AtomicBool::new(true);
    tokio::join!(
        handle_websocket(Platform::Bybit, Market::Futures, price_tx, &keep_running)
    );
}

#[allow(dead_code)]
#[allow(irrefutable_let_patterns)]
pub async fn kucoin_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    let keep_running = AtomicBool::new(true);
    tokio::join!(
        handle_websocket(Platform::Kucoin, Market::Futures, price_tx, &keep_running)
    );
}