use crate::{db::{self, get_db}, model};
use rust_decimal::Decimal;
use sqlx::{MySql, Pool, Transaction};

pub async fn get_all_arb_coin_prices() -> anyhow::Result<Vec<model::ArbCoinPrice>> {
    let diff_rate_list = sqlx::query_as::<_, model::ArbCoinPrice>(
        "select * from arb_coin_price",
    )
    .fetch_all(db::get_db()?.database())
    .await?;
    Ok(diff_rate_list)
}

pub async fn get_arb_coin_price_by_symbol(
    symbol: &String,
) -> anyhow::Result<model::ArbCoinPrice> {
    let coin_price_list = sqlx::query_as::<_, model::ArbCoinPrice>(
        "SELECT * FROM arb_coin_price WHERE symbol = ?",
    ).bind(symbol)
        .fetch_one(db::get_db()?.database())
        .await?;
    Ok(coin_price_list)
}

pub async fn update_arb_coin_price_by_id(
    id: i64,
    price_field: &str,
    price: Decimal,
    date_field: &str,
    updated: String,
) -> anyhow::Result<u64> {
    let query = format!("UPDATE arb_coin_price SET {} = ?, {} = ? WHERE id = ?", price_field, date_field);
    let rows = sqlx::query(&query)
        .bind(price)
        .bind(updated)
        .bind(id)
        .execute(db::get_db()?.database())
        .await?
        .rows_affected();
    Ok(rows)
}

pub async fn insert_arb_coin_price(coin_price: model::ArbCoinPrice) -> anyhow::Result<u64> {
    let last_insert_id = sqlx::query("INSERT INTO arb_coin_price (platform, symbol, binance_spot_price, binance_futures_price, bybit_futures_price, kucoin_futures_price, created, binance_updated, bybit_updated, kucoin_updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(coin_price.platform)
        .bind(coin_price.symbol)
        .bind(coin_price.binance_spot_price)
        .bind(coin_price.binance_futures_price)
        .bind(coin_price.bybit_futures_price)
        .bind(coin_price.kucoin_futures_price)
        .bind(coin_price.created)
        .bind(coin_price.binance_updated)
        .bind(coin_price.bybit_updated)
        .bind(coin_price.kucoin_updated)
        .execute(db::get_db()?.database())
        .await?
        .last_insert_id();
    Ok(last_insert_id)
}

pub async fn insert_arb_diff_signal(diff_signal: model::ArbDiffSignal) -> anyhow::Result<u64> {
    let last_insert_id = sqlx::query("INSERT INTO arb_diff_signal (symbol, from_compare, to_compare, price_diff, price_diff_rate, from_price, to_price, created, from_updated, to_updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(diff_signal.symbol)
        .bind(diff_signal.from_compare)
        .bind(diff_signal.to_compare)
        .bind(diff_signal.price_diff)
        .bind(diff_signal.price_diff_rate)
        .bind(diff_signal.from_price)
        .bind(diff_signal.to_price)
        .bind(diff_signal.created)
        .bind(diff_signal.from_updated)
        .bind(diff_signal.to_updated)
        .execute(db::get_db()?.database())
        .await?
        .last_insert_id();
    Ok(last_insert_id)
}

pub async fn get_arb_diff_signal_by_symbol_from_and_to_compare_v2(
    symbol: &String,
    from_compare: &String,
    to_compare: &String,
) -> anyhow::Result<model::ArbDiffSignal> {
    let diff_signal = sqlx::query_as::<_, model::ArbDiffSignal>(
        "SELECT * FROM arb_diff_signal WHERE symbol = ? AND from_compare = ? AND to_compare = ? AND state = NOT_STARTED",
    )
        .bind(symbol)
        .bind(from_compare)
        .bind(to_compare)
        .fetch_one(db::get_db()?.database())
        .await?;
    Ok(diff_signal)
}

pub async fn update_arb_diff_signal_by_id(
    id: i64,
    price_diff: Decimal,
    price_diff_rate: Decimal,
    from_price: Decimal,
    to_price: Decimal,
    updated: String,
    from_updated: String,
    to_updated: String
) -> anyhow::Result<u64> {
    let rows = sqlx::query("UPDATE arb_diff_signal SET price_diff = ?, price_diff_rate = ?, 
                                    from_price = ?, to_price = ?, updated = ?,
                                    from_updated = ?, to_updated = ? WHERE id = ?")
        .bind(price_diff)
        .bind(price_diff_rate)
        .bind(from_price)
        .bind(to_price)
        .bind(updated)
        .bind(from_updated)
        .bind(to_updated)
        .bind(id)
        .execute(db::get_db()?.database())
        .await?
        .rows_affected();
    Ok(rows)
}

// pub async fn delete_all_arb_coin_prices() -> anyhow::Result<u64> {
//     let rows_affected = sqlx::query("DELETE FROM arb_coin_price")
//         .execute(db::get_db()?.database())
//         .await?
//         .rows_affected();
//     Ok(rows_affected)
// }

pub async fn delete_all_arb_coin_prices() -> Result<()> {
    // let db = get_db()?;
    // let pool = db.database();

    // // Start a new transaction
    // let mut tx = pool.begin().await?;
    // let rows_affected = sqlx::query("DELETE FROM arb_coin_price")
    //     .execute(&mut tx)
    //     .await?
    //     .rows_affected();
    // Ok(rows_affected)

    let db = get_db()?;
        let pool = db.database();

        // Start a new transaction
        let mut tx = pool.begin().await?;

        let symbol = "BTCUSDT"; // Replace with dynamic symbol as needed

        // Fetch and process signals
        let records = sqlx::query_as::<_, (i32, String)>(
            "SELECT id, state FROM signal WHERE state = 'not started' OR state = 'open'"
        )
        .fetch_all(&mut tx) // Pass a mutable reference to tx
        .await?;

        for (id, state) in records {
            // Process logic here
            println!("Processing signal id: {}, state: {}", id, state);

            // Update state to 'closed' after processing
            sqlx::query("UPDATE signal SET state = 'closed' WHERE id = ?")
                .bind(id)
                .execute(&mut tx) // Pass a mutable reference to tx
                .await?;
        }

        // Commit the transaction
        tx.commit().await?;
}