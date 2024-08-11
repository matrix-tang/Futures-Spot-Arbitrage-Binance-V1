use crate::{db, model};
use rust_decimal::Decimal;

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
    let last_insert_id = sqlx::query("INSERT INTO arb_diff_signal (symbol, from_compare, to_compare, price_diff, price_diff_rate, binance_futures_price, bybit_futures_price, kucoin_futures_price, created, updated, binance_updated, bybit_updated, kucoin_updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(diff_signal.symbol)
        .bind(diff_signal.from_compare)
        .bind(diff_signal.to_compare)
        .bind(diff_signal.price_diff)
        .bind(diff_signal.price_diff_rate)
        .bind(diff_signal.binance_futures_price)
        .bind(diff_signal.bybit_futures_price)
        .bind(diff_signal.kucoin_futures_price)
        .bind(diff_signal.binance_updated)
        .bind(diff_signal.bybit_updated)
        .bind(diff_signal.kucoin_updated)
        .bind(diff_signal.created)
        .bind(diff_signal.updated)
        .execute(db::get_db()?.database())
        .await?
        .last_insert_id();
    Ok(last_insert_id)
}

pub async fn get_arb_diff_signal_by_symbol_from_and_to_compare(
    symbol: &String,
    from_compare: &String,
    to_compare: &String,
) -> anyhow::Result<model::ArbDiffSignal> {
    let diff_signal = sqlx::query_as::<_, model::ArbDiffSignal>(
        "SELECT * FROM arb_diff_signal WHERE symbol = ? AND from_compare = ? AND to_compare = ?",
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
    binance_futures_price: Decimal,
    bybit_futures_price: Decimal,
    kucoin_futures_price: Decimal,
    updated: String,
) -> anyhow::Result<u64> {
    let rows = sqlx::query("UPDATE arb_diff_signal SET price_diff = ?, price_diff_rate = ?, binance_futures_price = ?, bybit_futures_price = ?, kucoin_futures_price = ?, updated = ? WHERE id = ?")
        .bind(price_diff)
        .bind(price_diff_rate)
        .bind(binance_futures_price)
        .bind(bybit_futures_price)
        .bind(kucoin_futures_price)
        .bind(updated)
        .bind(id)
        .execute(db::get_db()?.database())
        .await?
        .rows_affected();
    Ok(rows)
}
