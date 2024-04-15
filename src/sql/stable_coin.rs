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
