#[macro_use]
extern crate tokio;

use arbitrage::conf;
use arbitrage::service::PriceStream;
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

    let streams: Vec<BoxFuture<'static, ()>> = vec![
        Box::pin(service::set_binance_price(price_rx)),
        Box::pin(service::spot_all_ticker(price_tx.clone())),
        Box::pin(service::futures_all_ticker(price_tx.clone())),
        Box::pin(service::delivery_all_ticker(price_tx.clone())),
        Box::pin(service::set_binance_diff_rate()),
        Box::pin(service::range_new_strategy()), //根据arb_strategy表创建arb_strategy_ex表
        Box::pin(service::inspect_strategy(txs.clone())), // 轮训策略
    ];

    for stream in streams {
        tokio::spawn(stream);
    }

    // 开始线程池
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
