use crate::{db, model};

pub async fn get_arb_stable_coin_list_by_doing_status(
    doing_status: i8,
) -> anyhow::Result<Vec<model::ArbStableCoin>> {
    let stable_coin_list = sqlx::query_as::<_, model::ArbStableCoin>(
        "select * from arb_stable_coin where doing_status = ?",
    )
    .bind(doing_status)
    .fetch_all(db::get_db()?.database())
    .await?;
    Ok(stable_coin_list)
}

pub async fn get_arb_stable_coin_info_list_by_stable_coin_id(
    stable_coin_id: i64,
    limit: u32,
) -> anyhow::Result<Vec<model::ArbStableCoinInfo>> {
    let info_list = sqlx::query_as::<_, model::ArbStableCoinInfo>(
        "select * from arb_stable_coin_info where stable_coin_id = ? order by id desc limit ?",
    )
    .bind(stable_coin_id)
    .bind(limit)
    .fetch_all(db::get_db()?.database())
    .await?;
    Ok(info_list)
}

pub async fn insert_arb_stable_coin_info(info: model::ArbStableCoinInfo) -> anyhow::Result<u64> {
    let last_insert_id = sqlx::query(
        "insert into arb_stable_coin_info (stable_coin_id, user_id, platform, coin, market, symbol, option_type, price, amount, order_id, is_ok, created) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(info.stable_coin_id)
        .bind(info.user_id)
        .bind(info.platform)
        .bind(info.coin)
        .bind(info.market)
        .bind(info.symbol)
        .bind(info.option_type)
        .bind(info.price)
        .bind(info.amount)
        .bind(info.order_id)
        .bind(info.is_ok)
        .bind(info.created)
        .execute(db::get_db()?.database())
        .await?
        .last_insert_id();
    Ok(last_insert_id)
}
