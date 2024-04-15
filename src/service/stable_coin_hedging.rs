use crate::binance::rest_model::{KlineSummaries, KlineSummary};
use crate::binance::MyApi;
use crate::{db, model, sql};
use anyhow::anyhow;
use log::error;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::collections::HashMap;
use ta::indicators::{BollingerBands, BollingerBandsOutput};
use ta::Next;
use tokio::select;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub async fn event_stable_coin_start(rxs: HashMap<i64, UnboundedReceiver<model::ArbStableCoin>>) {
    for (_, mut rx) in rxs {
        let api = MyApi::new();
        tokio::spawn(async move {
            loop {
                select! {
                    Some(stable_coin) = rx.recv() => {
                        match stable_coin.strategy.as_str() {
                            // boll 15m
                            "11" => {
                                if let Err(e) = boll(api.clone(), stable_coin).await {
                                    error!("boll err: {:?}", e);
                                }
                            },
                            // 百分比
                            "21" => {
                                if let Err(e) = percentage(api.clone(), stable_coin).await {
                                    error!("percentage err: {:?}", e);
                                }
                            },
                            // 固定阈值
                            "31" => {
                                if let Err(e) = fixed_threshold(api.clone(), stable_coin).await {
                                    error!("fixed threshold err: {:?}", e);
                                }
                            },
                            _ => {

                            }
                        }
                    }
                }
            }
        });
    }
}

async fn boll(api: MyApi, stable: model::ArbStableCoin) -> anyhow::Result<()> {
    // 初始化K线数据
    let kline_key = stable.symbol.clone() + "_15m";
    let kline_interval = "15m";

    let mut klines: Vec<KlineSummary> = Vec::new();
    if let Some(en) = db::get_db()?.rocksdb().get(kline_key.clone())? {
        klines = bincode::deserialize(&en[..])?;
    }

    if klines.len() == 0 {
        match api
            .get_klines(stable.symbol.clone(), kline_interval, 1000, None, None)
            .await
        {
            Ok(KlineSummaries::AllKlineSummaries(new_klines)) => {
                let encode = bincode::serialize(&new_klines)?;
                let _ = db::get_db()?.rocksdb().put(kline_key.clone(), encode)?;
            }
            Err(e) => {
                return Err(anyhow!(
                    "get {:?} klines error, : {:?}",
                    kline_key.clone(),
                    e.to_string()
                ));
            }
        }
    } else {
        // 增量K线
        if let Some(last) = klines.last() {
            match api
                .get_klines(
                    stable.symbol.clone(),
                    kline_interval,
                    5,
                    Some(last.open_time as u64),
                    None,
                )
                .await
            {
                Ok(KlineSummaries::AllKlineSummaries(new_klines)) => {
                    if new_klines.len() == 1 {
                        let mut k1: Vec<KlineSummary> = klines.clone();
                        k1.remove(klines.len() - 1);
                        k1.push(new_klines[0].clone());
                        let encode = bincode::serialize(&k1)?;
                        db::get_db()?.rocksdb().put(kline_key.clone(), encode)?;
                    } else {
                        let mut k2: Vec<KlineSummary> = klines.clone();
                        for (i, kline) in new_klines.iter().enumerate() {
                            if i == 0 {
                                k2.remove(klines.len() - 1);
                                k2.push(kline.clone());
                            } else {
                                k2.remove(0);
                                k2.push(kline.clone());
                            }
                        }
                        let encode = bincode::serialize(&k2)?;
                        db::get_db()?.rocksdb().put(kline_key.clone(), encode)?;
                    }
                }
                Err(e) => {
                    return Err(anyhow!(
                        "get last {:?} klines error, : {:?}",
                        kline_key.clone(),
                        e.to_string()
                    ));
                }
            }
        }
    }

    // 计算Boll
    let mut bb = BollingerBands::new(20, 2.0_f64)?;
    // let mut average = 0.0;
    let mut upper = 0.0;
    let mut lower = 0.0;
    for k in klines.clone() {
        let out = bb.next(k.close);
        // average = out.average;
        upper = out.upper;
        lower = out.lower;
    }

    let mut upp = Decimal::from_f64(upper).ok_or(anyhow!("decimal from f64 upp"))?;
    upp.rescale(stable.price_truncate as u32);
    /*let mut avg = Decimal::from_f64(average).ok_or(anyhow!(""))?;
    avg.rescale(stable.price_truncate as u32);*/
    let mut low = Decimal::from_f64(lower).ok_or(anyhow!("decimal from f64 low"))?;
    low.rescale(stable.price_truncate as u32);

    let price = klines.last().ok_or(anyhow!("last price"))?.close;
    let last_price = Decimal::from_f64(price).ok_or(anyhow!("decimal from f64 price"))?;

    // 策略，price < 1 && price <= lastDn buy -> price >= lastUp sell
    // 获取stable_coin_info 表最后1条数据状态

    println!(
        "up: {:?} dn: {:?} count: {:?} price: {:?}",
        upp,
        low,
        klines.len(),
        last_price
    );

    Ok(())
}

#[allow(dead_code, unused)]
async fn percentage(api: MyApi, stable: model::ArbStableCoin) -> anyhow::Result<()> {
    // println!("percentage: {:?}", stable);
    Ok(())
}

#[allow(dead_code, unused)]
async fn fixed_threshold(api: MyApi, stable: model::ArbStableCoin) -> anyhow::Result<()> {
    // println!("fixed_threshold: {:?}", stable);
    Ok(())
}

pub async fn inspect_stable_coin(txs: HashMap<i64, UnboundedSender<model::ArbStableCoin>>) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        match sql::get_arb_stable_coin_list_by_doing_status(
            model::arb_stable_coin::DOING_STATUS_RUN,
        )
        .await
        {
            Ok(strategy_list) => {
                for s in strategy_list {
                    let sharding = s.id % 10;
                    if let Some(tx) = txs.get(&sharding) {
                        if let Err(e) = tx.send(s.clone()) {
                            error!("send error: {:?}", e.to_string());
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
