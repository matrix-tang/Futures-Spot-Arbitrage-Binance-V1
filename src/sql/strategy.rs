use crate::{db, model};
use chrono::Local;
use std::collections::HashMap;

pub async fn update_strategy_by_id(id: i64, doing_status: i8) -> anyhow::Result<u64> {
    let rows = sqlx::query("update arb_strategy set doing_status = ?, updated = ? where id = ?")
        .bind(doing_status)
        .bind(Local::now().timestamp())
        .bind(id)
        .execute(db::get_db()?.database())
        .await?
        .rows_affected();
    Ok(rows)
}

pub async fn update_strategy_ex_by_id(
    id: i64,
    map: HashMap<String, String>,
) -> anyhow::Result<u64> {
    let sql_pre: String = map
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join(", ");
    let sql = format!(
        "update arb_strategy_ex set {}, updated = ? where id = ?",
        sql_pre
    );
    let rows = sqlx::query(&sql)
        .bind(Local::now().timestamp())
        .bind(id)
        .execute(db::get_db()?.database())
        .await?
        .rows_affected();
    Ok(rows)
}

pub async fn update_strategy_ex_info_by_id(
    id: i64,
    map: HashMap<String, String>,
) -> anyhow::Result<u64> {
    let sql_pre: String = map
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join(", ");
    let sql = format!(
        "update arb_strategy_ex_info set {}, updated = ? where id = ?",
        sql_pre
    );
    let rows = sqlx::query(&sql)
        .bind(Local::now().timestamp())
        .bind(id)
        .execute(db::get_db()?.database())
        .await?
        .rows_affected();
    Ok(rows)
}

pub async fn get_arb_strategy_list_by_doing_status(
    doing_status: i8,
) -> anyhow::Result<Vec<model::ArbStrategy>> {
    let strategy_list = sqlx::query_as::<_, model::ArbStrategy>(
        "select * from arb_strategy where doing_status = ?",
    )
    .bind(doing_status)
    .fetch_all(db::get_db()?.database())
    .await?;
    Ok(strategy_list)
}

pub async fn get_arb_strategy_ex_list_by_strategy_id(
    strategy_id: i64,
) -> anyhow::Result<Vec<model::ArbStrategyEx>> {
    let strategy_ex_list = sqlx::query_as::<_, model::ArbStrategyEx>(
        "select * from arb_strategy_ex where arb_strategy_id = ?",
    )
    .bind(strategy_id)
    .fetch_all(db::get_db()?.database())
    .await?;
    Ok(strategy_ex_list)
}

pub async fn get_arb_strategy_ex_info_by_order_id(
    order_id: String,
) -> anyhow::Result<model::ArbStrategyExInfo> {
    let ex_info = sqlx::query_as::<_, model::ArbStrategyExInfo>(
        "select * from arb_strategy_ex_info where order_id = ?",
    )
    .bind(order_id)
    .fetch_one(db::get_db()?.database())
    .await?;
    Ok(ex_info)
}

pub async fn insert_arb_strategy_ex(ex: model::ArbStrategyEx) -> anyhow::Result<u64> {
    let last_insert_id = sqlx::query("insert into arb_strategy_ex (user_id, platform, option_choose, arb_strategy_id, coin, market, symbol, option_type, option_status, option_amount, option_executed_amt, current_order_id, created, updated) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(ex.user_id)
        .bind(ex.platform)
        .bind(ex.option_choose)
        .bind(ex.arb_strategy_id)
        .bind(ex.coin)
        .bind(ex.market)
        .bind(ex.symbol)
        .bind(ex.option_type)
        .bind(ex.option_status)
        .bind(ex.option_amount)
        .bind(ex.option_executed_amt)
        .bind(ex.current_order_id)
        .bind(ex.created)
        .bind(ex.updated)
        .execute(db::get_db()?.database())
        .await?
        .last_insert_id();
    Ok(last_insert_id)
}

pub async fn insert_arb_strategy_ex_info(ex: model::ArbStrategyExInfo) -> anyhow::Result<u64> {
    let last_insert_id = sqlx::query(
        "insert into arb_strategy_ex_info (user_id, platform, option_choose, arb_strategy_id, arb_strategy_ex_id, coin, market, symbol, option_type, price, amount, executed_amt, order_id, is_ok, created) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
        .bind(ex.user_id)
        .bind(ex.platform)
        .bind(ex.option_choose)
        .bind(ex.arb_strategy_id)
        .bind(ex.arb_strategy_ex_id)
        .bind(ex.coin)
        .bind(ex.market)
        .bind(ex.symbol)
        .bind(ex.option_type)
        .bind(ex.price)
        .bind(ex.amount)
        .bind(ex.executed_amt)
        .bind(ex.order_id)
        .bind(ex.is_ok)
        .bind(ex.created)
        .execute(db::get_db()?.database())
        .await?
        .last_insert_id();
    Ok(last_insert_id)
}
