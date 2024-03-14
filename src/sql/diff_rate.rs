use crate::{db, model};
use chrono::Local;
use rust_decimal::Decimal;

pub async fn get_arb_diff_rate_list_by_diff_status(
    diff_status: i8,
) -> anyhow::Result<Vec<model::ArbDiffRate>> {
    let diff_rate_list = sqlx::query_as::<_, model::ArbDiffRate>(
        "select * from arb_diff_rate where diff_status = ?",
    )
    .bind(diff_status)
    .fetch_all(db::get_db()?.database())
    .await?;
    Ok(diff_rate_list)
}

pub async fn insert_arb_diff_rate_his(his: model::ArbDiffRateHis) -> anyhow::Result<u64> {
    let last_insert_id = sqlx::query("insert into arb_diff_rate_his (diff_rate_id, diff_price, diff_rate, created, updated) values (?, ?, ?, ?, ?)")
        .bind(his.diff_rate_id)
        .bind(his.diff_price)
        .bind(his.diff_rate)
        .bind(his.created)
        .bind(his.updated)
        .execute(db::get_db()?.database())
        .await?
        .last_insert_id();
    Ok(last_insert_id)
}

pub async fn get_arb_diff_rate_info_by_diff_rate_id(
    diff_rate_id: i64,
) -> anyhow::Result<model::ArbDiffRateInfo> {
    let diff_rate_info = sqlx::query_as::<_, model::ArbDiffRateInfo>(
        "select * from arb_diff_rate_info where diff_rate_id = ?",
    )
    .bind(diff_rate_id)
    .fetch_one(db::get_db()?.database())
    .await?;
    Ok(diff_rate_info)
}

pub async fn get_arb_strategy_ex_list_by_strategy_id(
    strategy_id: i64,
) -> anyhow::Result<Vec<model::ArbStrategyEx>> {
    let ex_list = sqlx::query_as::<_, model::ArbStrategyEx>(
        "select * from arb_strategy_ex where arb_strategy_id = ?",
    )
    .bind(strategy_id)
    .fetch_all(db::get_db()?.database())
    .await?;
    Ok(ex_list)
}

pub async fn update_arb_diff_rate_info_by_id(
    id: i64,
    from_price: Decimal,
    to_price: Decimal,
    diff_price: Decimal,
    diff_rate: Decimal,
) -> anyhow::Result<u64> {
    let rows = sqlx::query("update arb_diff_rate_info set from_price = ?, to_price = ?, diff_price = ?, diff_rate = ?, updated = ? where id = ?")
        .bind(from_price)
        .bind(to_price)
        .bind(diff_price)
        .bind(diff_rate)
        .bind(Local::now().timestamp())
        .bind(id)
        .execute(db::get_db()?.database())
        .await?
        .rows_affected();
    Ok(rows)
}

pub async fn insert_arb_diff_rate_info(info: model::ArbDiffRateInfo) -> anyhow::Result<u64> {
    let last_insert_id = sqlx::query(
        "insert into arb_diff_rate_info (
        diff_rate_id,
        platform,
        coin,
        option_choose,
        from_market,
        from_symbol,
        from_price,
        to_market,
        to_symbol,
        to_price,
        diff_price,
        diff_rate,
        created,
        updated
        ) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(info.diff_rate_id)
    .bind(info.platform)
    .bind(info.coin)
    .bind(info.option_choose)
    .bind(info.from_market)
    .bind(info.from_symbol)
    .bind(info.from_price)
    .bind(info.to_market)
    .bind(info.to_symbol)
    .bind(info.to_price)
    .bind(info.diff_price)
    .bind(info.diff_rate)
    .bind(info.created)
    .bind(info.updated)
    .execute(db::get_db()?.database())
    .await?
    .last_insert_id();
    Ok(last_insert_id)
}
