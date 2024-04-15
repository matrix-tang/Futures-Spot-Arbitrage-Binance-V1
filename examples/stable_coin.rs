use arbitrage::{conf, db, helper, model, sql};
use chrono::Local;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化配置文件
    lazy_static::initialize(&conf::C);
    // 初始化Db
    db::init_env().await?;
    // 初始化日志
    helper::log::init_log();

    let last_id = sql::insert_arb_stable_coin_info(model::ArbStableCoinInfo {
        id: 0,
        stable_coin_id: 1,
        user_id: 6,
        platform: "binance".to_string(),
        coin: "FDUSD".to_string(),
        market: "spot".to_string(),
        symbol: "FDUSDUSDT".to_string(),
        option_type: "buy".to_string(),
        price: Decimal::from_f64(0.9999).unwrap(),
        amount: Decimal::from_f64(500.0).unwrap(),
        order_id: "1357901234".to_string(),
        is_ok: model::arb_stable_coin_info::IS_OK_COMPLETED,
        created: Some(Local::now().timestamp()),
        updated: None,
        bak: None,
    })
    .await?;
    println!("last_id: {:?}", last_id);

    Ok(())
}
