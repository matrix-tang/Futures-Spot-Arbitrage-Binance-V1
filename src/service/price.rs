use crate::binance::ws_model::BinanceTickerEvent;
use crate::conf::redis_key;
use crate::db;
use crate::service::PriceStream;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use tokio::select;
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceInfo {
    pub ticker: BinanceTickerEvent,
    pub market: String,
}

pub async fn set_binance_price(mut price_rx: UnboundedReceiver<PriceStream>) {
    let mut redis = db::get_db().unwrap().redis().await.unwrap();
    loop {
        select! {
            event = price_rx.recv() => {
                if let Some(stream)  = event {
                    // println!("{:?} {:?}", stream.market, stream.local_time);
                    let key = format!("{}{}", stream.market.as_str(), redis_key::BINANCE_PRICE_KEY);

                    let mut items = vec![];
                    for ticker in stream.tickers {
                        let ticker_json = serde_json::to_string(&ticker).unwrap();
                        items.push((ticker.symbol, ticker_json))
                    }
                    let _: () = redis.hset_multiple(key, &items).await.unwrap();
                    // info!("--------------, set binance {:?} price, current time: {:?}", stream.market, Local::now().timestamp_millis());
                }
            },
        }
    }
}

pub async fn get_binance_price(market: String, symbol: String) -> anyhow::Result<PriceInfo> {
    let mut redis = db::get_db()?.redis().await?;

    let key = format!("{}{}", market, redis_key::BINANCE_PRICE_KEY);
    let x: String = redis.hget(key, symbol).await?;
    let ticker = serde_json::from_str::<BinanceTickerEvent>(x.as_str()).unwrap();
    let info = PriceInfo { ticker, market };

    Ok(info)
}
