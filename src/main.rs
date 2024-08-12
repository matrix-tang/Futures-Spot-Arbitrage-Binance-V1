#[macro_use]
extern crate tokio;

use arbitrage::conf;
use arbitrage::service::PriceStream;
use arbitrage::sql::delete_all_arb_coin_prices;
use arbitrage::{db, helper, service};
use futures::future::BoxFuture;
use log::warn;
use std::collections::HashMap;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化配置文件
    lazy_static::initialize(&conf::C);
    // 初始化Db
    db::init_env().await?;
    
    // Call the delete_all_arb_coin_prices function
    delete_all_arb_coin_prices().await?;
    // 初始化日志
    helper::log::init_log();

    let (close_tx, mut close_rx) = tokio::sync::mpsc::unbounded_channel::<bool>();
    let (price_tx, price_rx) = tokio::sync::mpsc::unbounded_channel::<PriceStream>();

    let wait_loop = tokio::spawn(async move {
        'hello: loop {
            select! {
                _ = close_rx.recv() => break 'hello
            }
        }
    });
    // 线程池通道
    let mut txs = HashMap::new();
    let mut rxs = HashMap::new();
    for i in 0..10 {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        txs.insert(i, tx.clone());
        rxs.insert(i, rx);
    }

    // let funding_rate_json = r#"{"topic":"/contract/instrument:ALL","type":"message","subject":"funding.rate","data":{"symbol":"IDUSDTM","granularity":60000,"fundingRate":-0.000880,"timestamp":1722976560000}}"#;
    // match serde_json::from_str::<WebSocketEvent>(funding_rate_json) {
    //     Ok(event) => match event {
    //         WebSocketEvent::KucoinTicker(funding_rate_message) => {
    //             println!("Received Funding Rate Message: {:?}", funding_rate_message);
    //         }
    //         // Handle other variants if needed
    //         _ => println!("Received a different type of WebSocket event."),
    //     },
    //     Err(e) => println!("Failed to deserialize JSON: {}", e),
    // }
    let streams: Vec<BoxFuture<'static, ()>> = vec![
        Box::pin(service::set_price_data(price_rx)),
        Box::pin(service::binance_all_ticker(price_tx.clone())),
        // Box::pin(service::bybit_all_ticker(price_tx.clone())),
        // Box::pin(service::kucoin_all_ticker(price_tx.clone())),
        // Box::pin(service::get_diff_signal()),
        // Box::pin(service::range_new_strategy()), //根据arb_strategy表创建arb_strategy_ex表
        // Box::pin(service::inspect_strategy(txs.clone())), // 轮训策略
    ];

    for stream in streams {
        tokio::spawn(stream);
    }

    //开始线程池
    service::event_start(rxs).await;

    select! {
        _ = wait_loop => { warn!("Finished!") }
        _ = tokio::signal::ctrl_c() => {
            warn!("Closing websocket stream...");
            close_tx.send(true).unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    Ok(())
}




