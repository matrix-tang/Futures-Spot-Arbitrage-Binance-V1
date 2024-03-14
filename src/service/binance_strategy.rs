use crate::binance::api::{
    FuturesGetOrderRequest, FuturesOrderRequest, OrderRequest, OrderStatusRequest,
};
use crate::binance::rest_model::{
    OrderSide, OrderStatus, OrderType, TimeInForce, UniversalTransferType,
};
use crate::binance::MyApi;
use crate::service::common;
use crate::{model, sql};
use anyhow::anyhow;
use chrono::Local;
use log::{error, info, warn};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};
use tokio::select;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub async fn event_start(rxs: HashMap<i64, UnboundedReceiver<model::ArbStrategy>>) {
    for (_, mut rx) in rxs {
        let api = MyApi::new();
        tokio::spawn(async move {
            loop {
                select! {
                    Some(strategy) = rx.recv() => {
                        match strategy.option_choose.as_str() {
                            // 逻辑处理 正向positive， 差价比率 >= 0.05 现货spot买入 -> transfer到币本期货 -> delivery卖出，
                            // 差价比率 <= 0 delivery买入 -> transfer到现货 -> 现货spot卖出
                            "positive" => {
                                if let Err(e) = positive(api.clone(), strategy).await {
                                    error!("positive err: {:?}", e);
                                }
                            },
                            // 逻辑处理 反向reverse, 差价比率 <= -0.05 U本位: 远期futures买入 -> futures永续卖出 -> 差价比率 >= 0.0 futures永续买入 -> 远期futures卖出,
                            // 币本位: 远期delivery买入 -> delivery永续卖出 -> 差价比率 >= 0.0 delivery永续买入 -> 远期delivery卖出
                            "reverse" => {
                                if let Err(e) = reverse(api.clone(), strategy).await {
                                    error!("positive err: {:?}", e);
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

async fn positive(api: MyApi, strategy: model::ArbStrategy) -> anyhow::Result<()> {
    info!("positive: {:?}", strategy.id);
    // 获取执行策略列表
    let arb_ex_list = sql::get_arb_strategy_ex_list_by_strategy_id(strategy.id).await?;
    if arb_ex_list.is_empty() {
        return Err(anyhow!(
            "get arb_strategy_ex list err, by strategy_id: {:?}",
            strategy.id
        ));
    }
    // 检查策略数量 >6 错误
    if arb_ex_list.len() != 6 {
        return Err(anyhow!("arb count err: {:?}", arb_ex_list.len()));
    }

    let mut arb_ex_map = HashMap::new();
    let mut arb_strategy_done_count = 0;
    for ex in arb_ex_list {
        arb_ex_map.insert(ex.clone().option_type, ex.clone());
        if ex.option_status == model::arb_strategy_ex::OPTION_STATUS_DONE {
            arb_strategy_done_count += 1;
        }
    }
    // 判断当前策略是否已经完成
    if arb_strategy_done_count == 6 {
        let _ =
            sql::update_strategy_by_id(strategy.id, model::arb_strategy::DOING_STATUS_DONE).await?;
        return Err(anyhow!(
            "current strategy already done, strategy_id: {:?}",
            strategy.id
        ));
    }

    // 开仓
    let diff_rate_info = sql::get_arb_diff_rate_info_by_diff_rate_id(strategy.diff_rate_id).await?;
    if diff_rate_info.diff_rate >= strategy.option_open {
        // 1、from market buy 买入现货
        let from_market_buy_ex = arb_ex_map
            .get("spot_buy")
            .ok_or(anyhow!("get arb_ex_map spot_buy error"))?;
        if from_market_buy_ex.option_status != model::arb_strategy_ex::OPTION_STATUS_DONE {
            // 判断执行顺序
            if arb_strategy_done_count != 0 {
                return Err(anyhow!(
                    "done count err, spot buy, count: {}",
                    arb_strategy_done_count
                ));
            }

            let mut price = diff_rate_info.from_price.add(strategy.fok_diff);
            price.rescale(strategy.from_price_truncate as u32);
            let mut amount = from_market_buy_ex.option_amount;
            amount.rescale(strategy.from_amt_truncate as u32);
            let _ = spot_order_update(
                api,
                strategy.from_symbol.clone(),
                OrderSide::Buy,
                OrderType::Limit,
                "spot_buy".to_string(),
                price,
                amount,
                &strategy,
                from_market_buy_ex,
            )
            .await?;

            return Ok(());
        }

        // transfer spot to delivery 划转币现货到币本位交割期货
        let transfer_spot_to_delivery_ex = arb_ex_map
            .get("transfer_spot_to_delivery")
            .ok_or(anyhow!("get arb_ex_map transfer_spot_to_delivery error"))?;
        if transfer_spot_to_delivery_ex.option_status != model::arb_strategy_ex::OPTION_STATUS_DONE
        {
            // 判断执行顺序
            if arb_strategy_done_count != 1 {
                return Err(anyhow!(
                    "done count err, transfer_spot_to_delivery, count: {}",
                    arb_strategy_done_count
                ));
            }

            let mut amount = from_market_buy_ex
                .option_executed_amt
                .mul(Decimal::from(1).sub(strategy.spot_fee));
            amount.rescale(strategy.from_amt_truncate as u32);
            let _ = transfer_coin(
                api,
                strategy.coin.clone(),
                UniversalTransferType::MainCmfuture,
                amount,
                "transfer_spot_to_delivery".to_string(),
                &strategy,
                transfer_spot_to_delivery_ex,
            )
            .await?;

            return Ok(());
        }

        // to market sell 卖出币本位交割
        let to_market_sell_ex = arb_ex_map
            .get("delivery_sell")
            .ok_or(anyhow!("get arb_ex_map delivery_sell error"))?;
        if to_market_sell_ex.option_status != model::arb_strategy_ex::OPTION_STATUS_DONE {
            // 判断执行顺序
            if arb_strategy_done_count != 2 {
                return Err(anyhow!(
                    "done count err, delivery sell, count: {}",
                    arb_strategy_done_count
                ));
            }

            // 计算可开张数
            let cont = transfer_spot_to_delivery_ex
                .option_executed_amt
                .mul(diff_rate_info.to_price)
                .div(Decimal::from(strategy.contract_mul));
            let contract_num = cont.ceil().sub(Decimal::from(1));
            let mut price = diff_rate_info.to_price.sub(strategy.fok_diff);
            price.rescale(strategy.to_price_truncate as u32);
            let _ = delivery_order_update(
                api,
                strategy.to_symbol.clone(),
                OrderSide::Sell,
                OrderType::Limit,
                "delivery_sell".to_string(),
                price,
                contract_num,
                &strategy,
                to_market_sell_ex,
            )
            .await?;

            return Ok(());
        }
    }

    // 平仓
    if diff_rate_info.diff_rate <= strategy.option_close {
        // to market buy 买入币本位远期期货
        let to_market_buy_ex = arb_ex_map
            .get("delivery_buy")
            .ok_or(anyhow!("get arb_ex_map delivery_buy error"))?;
        if to_market_buy_ex.option_status != model::arb_strategy_ex::OPTION_STATUS_DONE {
            // 判断执行顺序
            if arb_strategy_done_count != 3 {
                return Err(anyhow!(
                    "done count err, delivery buy, count: {}",
                    arb_strategy_done_count
                ));
            }

            // 获取卖出张数
            let to_market_sell_ex = arb_ex_map
                .get("delivery_sell")
                .ok_or(anyhow!("get arb_ex_map delivery_sell error"))?;

            let mut price = diff_rate_info.to_price.add(strategy.fok_diff);
            price.rescale(strategy.to_price_truncate as u32);
            let _ = delivery_order_update(
                api,
                strategy.to_symbol.clone(),
                OrderSide::Buy,
                OrderType::Limit,
                "delivery_buy".to_string(),
                price,
                to_market_sell_ex.option_executed_amt,
                &strategy,
                to_market_buy_ex,
            )
            .await?;

            return Ok(());
        }

        // transfer delivery to spot  划转币现货到币本位交割期货
        let transfer_delivery_to_spot_ex = arb_ex_map
            .get("transfer_delivery_to_spot")
            .ok_or(anyhow!("get arb_ex_map transfer_delivery_to_spot error"))?;
        if transfer_delivery_to_spot_ex.option_status != model::arb_strategy_ex::OPTION_STATUS_DONE
        {
            // 判断执行顺序
            if arb_strategy_done_count != 4 {
                return Err(anyhow!(
                    "done count err, transfer_delivery_to_spot, count: {}",
                    arb_strategy_done_count
                ));
            }

            // 计算可划转数量
            let order = api
                .delivery_order_status(FuturesGetOrderRequest {
                    symbol: strategy.to_symbol.clone(),
                    order_id: Some(to_market_buy_ex.current_order_id.clone()),
                    orig_client_order_id: None,
                })
                .await?;
            let mut amount = Decimal::from_f64(order.cum_base)
                .ok_or(anyhow!(""))?
                .mul(Decimal::from(1).sub(strategy.delivery_fee));
            amount.rescale(strategy.to_amt_truncate as u32);
            let _ = transfer_coin(
                api,
                strategy.coin.clone(),
                UniversalTransferType::CmfutureMain,
                amount,
                "transfer_spot_to_delivery".to_string(),
                &strategy,
                transfer_delivery_to_spot_ex,
            )
            .await?;

            return Ok(());
        }

        // from market sell 卖出现货
        let from_market_sell_ex = arb_ex_map
            .get("spot_sell")
            .ok_or(anyhow!("get arb_ex_map spot_sell error"))?;
        if from_market_sell_ex
            .option_status
            // transfer spot to delivery 划转币现货到币本位交割期货
            != model::arb_strategy_ex::OPTION_STATUS_DONE
        {
            // 判断执行顺序
            if arb_strategy_done_count != 5 {
                return Err(anyhow!(
                    "done count err, spot_sell, count: {}",
                    arb_strategy_done_count
                ));
            }

            let mut price = diff_rate_info.from_price.sub(strategy.fok_diff);
            price.rescale(strategy.from_price_truncate as u32);
            let mut amount = transfer_delivery_to_spot_ex.option_executed_amt;
            amount.rescale(strategy.from_amt_truncate as u32);
            let _ = spot_order_update(
                api,
                strategy.from_symbol.clone(),
                OrderSide::Sell,
                OrderType::Limit,
                "spot_sell".to_string(),
                price,
                amount,
                &strategy,
                from_market_sell_ex,
            )
            .await?;

            return Ok(());
        }
    }

    Ok(())
}

async fn reverse(api: MyApi, strategy: model::ArbStrategy) -> anyhow::Result<()> {
    info!("reverse: {:?}", strategy.id);
    // 获取执行策略列表
    let arb_ex_list = sql::get_arb_strategy_ex_list_by_strategy_id(strategy.id).await?;
    if arb_ex_list.is_empty() {
        return Err(anyhow!(
            "get arb_strategy_ex list err, by strategy_id: {:?}",
            strategy.id
        ));
    }
    // 检查策略数量 >6 错误
    if arb_ex_list.len() != 4 {
        return Err(anyhow!("arb count err: {:?}", arb_ex_list.len()));
    }

    let mut arb_ex_map = HashMap::new();
    let mut arb_strategy_done_count = 0;
    for ex in arb_ex_list {
        let key = format!("{}-{}", ex.option_type, ex.symbol);
        arb_ex_map.insert(key, ex.clone());
        if ex.option_status == model::arb_strategy_ex::OPTION_STATUS_DONE {
            arb_strategy_done_count += 1;
        }
    }
    info!("{:?}", arb_ex_map.keys());
    // 判断当前策略是否已经完成
    if arb_strategy_done_count == 4 {
        let _ =
            sql::update_strategy_by_id(strategy.id, model::arb_strategy::DOING_STATUS_DONE).await?;
        return Err(anyhow!(
            "current strategy already done, strategy_id: {:?}",
            strategy.id
        ));
    }

    let diff_rate_info = sql::get_arb_diff_rate_info_by_diff_rate_id(strategy.diff_rate_id).await?;
    // 判断反向套利，是币本位还是U本位
    // USDM U本位
    if strategy.from_market == "futures" && strategy.to_market == "futures" {
        // 开仓
        if diff_rate_info.diff_rate <= strategy.option_open {
            // from market buy 买入远期
            let from_market_buy = format!("{}_buy-{}", strategy.from_market, strategy.from_symbol);
            // to market sell 卖出永续
            let to_market_sell = format!("{}_sell-{}", strategy.to_market, strategy.to_symbol);
            info!("open: {}, {}", from_market_buy, to_market_sell);
        }
        // 平仓
        if diff_rate_info.diff_rate >= strategy.option_close {
            // to market buy 买入永续
            let to_market_buy = format!("{}_buy-{}", strategy.to_market, strategy.to_symbol);
            // from market sell 卖出远期
            let from_market_sell =
                format!("{}_sell-{}", strategy.from_market, strategy.from_symbol);
            info!("close: {}, {}", to_market_buy, from_market_sell);
        }
    } else if strategy.from_market == "delivery" && strategy.to_market == "delivery" {
        // COINM 币本位
        // 开仓
        if diff_rate_info.diff_rate <= strategy.option_open {
            // from market buy 买入远期
            let from_market_buy = format!("{}_buy-{}", strategy.from_market, strategy.from_symbol);
            // to market sell 卖出永续
            let to_market_sell = format!("{}_sell-{}", strategy.to_market, strategy.to_symbol);
            info!("open: {}, {}", from_market_buy, to_market_sell);
        }
        // 平仓
        if diff_rate_info.diff_rate >= strategy.option_close {
            // to market buy 买入永续
            let to_market_buy = format!("{}_buy-{}", strategy.to_market, strategy.to_symbol);
            // from market sell 卖出远期
            let from_market_sell =
                format!("{}_sell-{}", strategy.from_market, strategy.from_symbol);
            info!("close: {}, {}", to_market_buy, from_market_sell);
        }
    }

    Ok(())
}

async fn spot_order_update(
    api: MyApi,
    symbol: String,
    order_side: OrderSide,
    order_type: OrderType,
    option_type: String,
    price: Decimal,
    amount: Decimal,
    strategy: &model::ArbStrategy,
    ex: &model::ArbStrategyEx,
) -> anyhow::Result<()> {
    // 下单处理
    if ex.current_order_id.is_empty() {
        // 下单
        let transaction = api
            .place_order(OrderRequest {
                symbol: symbol.clone(),
                quantity: Some(amount.to_f64().ok_or(anyhow!(""))?),
                price: Some(price.to_f64().ok_or(anyhow!(""))?),
                order_type: order_type.clone(),
                side: order_side.clone(),
                time_in_force: Some(TimeInForce::FOK),
                ..OrderRequest::default()
            })
            .await?;
        warn!("strategy_id: {}, {} place order, symbol: {}, side: {:?}, order_type: {:?}, amount: {}, price: {}, order_id: {}",
			strategy.id, option_type.clone(), symbol.clone(), order_side.clone(), order_type.clone(), amount, price, transaction.order_id);
        // 更新订单ID
        let mut data = HashMap::new();
        data.insert(
            "current_order_id".to_string(),
            transaction.order_id.to_string(),
        );
        let _ = sql::update_strategy_ex_by_id(ex.id, data).await?;

        // 插入详情表
        let _ = sql::insert_arb_strategy_ex_info(model::ArbStrategyExInfo {
            id: 0,
            user_id: strategy.user_id,
            platform: strategy.platform.clone(),
            option_choose: strategy.option_choose.clone(),
            arb_strategy_id: strategy.id,
            arb_strategy_ex_id: ex.id,
            coin: strategy.coin.clone(),
            market: ex.market.clone(),
            symbol: ex.symbol.clone(),
            option_type: option_type,
            price,
            amount,
            executed_amt: Decimal::ZERO,
            order_id: transaction.order_id.to_string(),
            is_ok: model::arb_strategy_ex_info::IS_OK_UN_DONE,
            created: Some(Local::now().timestamp()),
            updated: None,
            bak: None,
        })
        .await?;
    } else {
        // 已经下单处理
        let order = api
            .order_status(OrderStatusRequest {
                symbol,
                order_id: Some(ex.current_order_id.clone().parse::<u64>()?),
                orig_client_order_id: None,
                recv_window: None,
            })
            .await?;

        let ex_info =
            sql::get_arb_strategy_ex_info_by_order_id(ex.current_order_id.clone()).await?;
        // info!("{:?} {:?}", order, ex_info);

        if order.status == OrderStatus::Filled {
            let mut ex_data = HashMap::new();
            ex_data.insert("option_amount".to_string(), order.executed_qty.to_string());
            ex_data.insert(
                "option_executed_amt".to_string(),
                order.executed_qty.to_string(),
            );
            ex_data.insert(
                "option_status".to_string(),
                model::arb_strategy_ex::OPTION_STATUS_DONE.to_string(),
            );
            let _ = sql::update_strategy_ex_by_id(ex.id, ex_data).await?;

            let mut ex_info_data = HashMap::new();
            ex_info_data.insert("executed_amt".to_string(), order.executed_qty.to_string());
            ex_info_data.insert(
                "is_ok".to_string(),
                model::arb_strategy_ex_info::IS_OK_DONE.to_string(),
            );
            let _ = sql::update_strategy_ex_info_by_id(ex_info.id, ex_info_data).await?;
        } else {
            // 订单未立即全部成交，取消
            info!("order not filled, canceled");
            let mut ex_data = HashMap::new();
            ex_data.insert("current_order_id".to_string(), "".to_string());
            let _ = sql::update_strategy_ex_by_id(ex.id, ex_data).await?;

            let mut ex_info_data = HashMap::new();
            ex_info_data.insert(
                "is_ok".to_string(),
                model::arb_strategy_ex_info::IS_OK_EXPIRED.to_string(),
            );
            let _ = sql::update_strategy_ex_info_by_id(ex_info.id, ex_info_data).await?;
        }
    }

    Ok(())
}

pub async fn delivery_order_update(
    api: MyApi,
    symbol: String,
    order_side: OrderSide,
    order_type: OrderType,
    option_type: String,
    price: Decimal,
    amount: Decimal,
    strategy: &model::ArbStrategy,
    ex: &model::ArbStrategyEx,
) -> anyhow::Result<()> {
    // 下单处理
    if ex.current_order_id.is_empty() {
        // 下单
        let transaction = api
            .delivery_place_order(FuturesOrderRequest {
                symbol: symbol.clone(),
                side: order_side.clone(),
                order_type: order_type.clone(),
                quantity: Some(amount.to_f64().ok_or(anyhow!(""))?),
                price: Some(price.to_f64().ok_or(anyhow!(""))?),
                time_in_force: Some(TimeInForce::FOK),
                recv_window: None,
            })
            .await?;

        warn!("strategy_id: {}, {} place order, symbol: {}, side: {:?}, order_type: {:?}, amount: {}, price: {}, order_id: {}",
			strategy.id, option_type.clone(), symbol.clone(), order_side.clone(), order_type.clone(), amount, price, transaction.order_id);
        // 更新订单ID
        let mut data = HashMap::new();
        data.insert(
            "current_order_id".to_string(),
            transaction.order_id.to_string(),
        );
        let _ = sql::update_strategy_ex_by_id(ex.id, data).await?;

        // 插入详情表
        let _ = sql::insert_arb_strategy_ex_info(model::ArbStrategyExInfo {
            id: 0,
            user_id: strategy.user_id,
            platform: strategy.platform.clone(),
            option_choose: strategy.option_choose.clone(),
            arb_strategy_id: strategy.id,
            arb_strategy_ex_id: ex.id,
            coin: strategy.coin.clone(),
            market: ex.market.clone(),
            symbol: ex.symbol.clone(),
            option_type: option_type,
            price,
            amount,
            executed_amt: Decimal::ZERO,
            order_id: transaction.order_id.to_string(),
            is_ok: model::arb_strategy_ex_info::IS_OK_UN_DONE,
            created: Some(Local::now().timestamp()),
            updated: None,
            bak: None,
        })
        .await?;
    } else {
        // 已经下单处理
        let order = api
            .delivery_order_status(FuturesGetOrderRequest {
                symbol: symbol,
                order_id: Some(ex.current_order_id.clone()),
                orig_client_order_id: None,
            })
            .await?;

        let ex_info =
            sql::get_arb_strategy_ex_info_by_order_id(ex.current_order_id.clone()).await?;
        // info!("{:?} {:?}", order, ex_info);

        if order.status == "FILLED".to_string() {
            let mut ex_data = HashMap::new();
            ex_data.insert("option_amount".to_string(), order.executed_qty.to_string());
            ex_data.insert(
                "option_executed_amt".to_string(),
                order.executed_qty.to_string(),
            );
            ex_data.insert(
                "option_status".to_string(),
                model::arb_strategy_ex::OPTION_STATUS_DONE.to_string(),
            );
            let _ = sql::update_strategy_ex_by_id(ex.id, ex_data).await?;

            let mut ex_info_data = HashMap::new();
            ex_info_data.insert("executed_amt".to_string(), order.executed_qty.to_string());
            ex_info_data.insert(
                "is_ok".to_string(),
                model::arb_strategy_ex_info::IS_OK_DONE.to_string(),
            );
            let _ = sql::update_strategy_ex_info_by_id(ex_info.id, ex_info_data).await?;
        } else {
            // 订单未立即全部成交，取消
            info!("order not filled, canceled");
            let mut ex_data = HashMap::new();
            ex_data.insert("current_order_id".to_string(), "".to_string());
            let _ = sql::update_strategy_ex_by_id(ex.id, ex_data).await?;

            let mut ex_info_data = HashMap::new();
            ex_info_data.insert(
                "is_ok".to_string(),
                model::arb_strategy_ex_info::IS_OK_EXPIRED.to_string(),
            );
            let _ = sql::update_strategy_ex_info_by_id(ex_info.id, ex_info_data).await?;
        }
    }
    Ok(())
}

pub async fn transfer_coin(
    api: MyApi,
    coin: String,
    transfer_type: UniversalTransferType,
    amount: Decimal,
    option_type: String,
    strategy: &model::ArbStrategy,
    ex: &model::ArbStrategyEx,
) -> anyhow::Result<()> {
    let transfer = api
        .universal_transfer(
            coin.clone(),
            amount.to_f64().ok_or(anyhow!(""))?,
            transfer_type,
        )
        .await?;
    warn!(
        "strategy_id: {}, {}, coin: {}, amount: {}, transfer_id: {}",
        strategy.id, option_type, coin, amount, transfer.tran_id
    );

    // 插入详情表
    let _ = sql::insert_arb_strategy_ex_info(model::ArbStrategyExInfo {
        id: 0,
        user_id: strategy.user_id,
        platform: strategy.platform.clone(),
        option_choose: strategy.option_choose.clone(),
        arb_strategy_id: strategy.id,
        arb_strategy_ex_id: ex.id,
        coin: strategy.coin.clone(),
        market: ex.market.clone(),
        symbol: ex.symbol.clone(),
        option_type: option_type,
        price: Decimal::ZERO,
        amount,
        executed_amt: amount,
        order_id: transfer.tran_id.to_string(),
        is_ok: model::arb_strategy_ex_info::IS_OK_DONE,
        created: Some(Local::now().timestamp()),
        updated: Some(Local::now().timestamp()),
        bak: None,
    })
    .await?;

    let mut ex_data = HashMap::new();
    ex_data.insert("current_order_id".to_string(), transfer.tran_id.to_string());
    ex_data.insert(
        "option_status".to_string(),
        model::arb_strategy_ex::OPTION_STATUS_DONE.to_string(),
    );
    ex_data.insert("option_amount".to_string(), amount.to_string());
    ex_data.insert("option_executed_amt".to_string(), amount.to_string());
    let _ = sql::update_strategy_ex_by_id(ex.id, ex_data).await?;

    Ok(())
}

pub async fn range_new_strategy() {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        match sql::get_arb_strategy_list_by_doing_status(model::arb_strategy::DOING_STATUS_RUN)
            .await
        {
            Ok(strategy_list) => {
                for strategy in strategy_list {
                    // 获取预执行策略
                    if let Ok(ex_list) =
                        sql::get_arb_strategy_ex_list_by_strategy_id(strategy.id).await
                    {
                        // 如果策略不为空跳过
                        if !ex_list.is_empty() {
                            continue;
                        }
                    }

                    let mut ex_desc_map = HashMap::new();
                    let mut count = 0;
                    if strategy.option_choose == "positive" {
                        if let Ok(ex_desc) = common::new_positive_desc(strategy.clone()) {
                            ex_desc_map = ex_desc;
                            count = 6;
                        }
                    } else if strategy.option_choose == "reverse" {
                        if strategy.from_market == "delivery" && strategy.to_market == "delivery" {
                            if let Ok(ex_desc) = common::new_reverse_coinm_desc(strategy.clone()) {
                                ex_desc_map = ex_desc;
                                count = 4;
                            }
                        } else if strategy.from_market == "futures"
                            && strategy.to_market == "futures"
                        {
                            if let Ok(ex_desc) = common::new_reverse_usdm_desc(strategy.clone()) {
                                ex_desc_map = ex_desc;
                                count = 4;
                            }
                        }
                    }

                    for i in 0..count {
                        if let Some(ex) = ex_desc_map.get(&i) {
                            // ex
                            if let Ok(last_id) = sql::insert_arb_strategy_ex(model::ArbStrategyEx {
                                id: 0,
                                user_id: strategy.user_id.clone(),
                                platform: strategy.platform.clone(),
                                option_choose: strategy.option_choose.clone(),
                                arb_strategy_id: strategy.id.clone(),
                                coin: strategy.coin.clone(),
                                market: ex.clone().market,
                                symbol: ex.clone().symbol,
                                option_type: ex.clone().option_type,
                                option_status: model::arb_strategy_ex::OPTION_STATUS_UN_DONE,
                                option_amount: strategy
                                    .option_amt
                                    .clone()
                                    .mul(Decimal::from(strategy.margin_mul.clone())),
                                option_executed_amt: Decimal::ZERO,
                                current_order_id: "".to_string(),
                                created: Some(Local::now().timestamp()),
                                updated: Some(Local::now().timestamp()),
                                bak: None,
                            })
                            .await
                            {
                                info!("insert arb_strategy_ex id: {:?}", last_id);
                            }
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

pub async fn inspect_strategy(txs: HashMap<i64, UnboundedSender<model::ArbStrategy>>) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        match sql::get_arb_strategy_list_by_doing_status(model::arb_strategy::DOING_STATUS_RUN)
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
