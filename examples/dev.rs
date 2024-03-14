use arbitrage::{conf, db, model};
use redis::AsyncCommands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化配置文件
    lazy_static::initialize(&conf::C);

    println!(
        "{:?} {:?}",
        conf::C.binance_api_config.api_key,
        conf::C.mysql.url
    );

    let env = db::Db::new().await?;

    let data = sqlx::query_as::<_, model::ArbDiffRate>("select * from arb_diff_rate")
        .fetch_all(env.database())
        .await?;
    println!("{:?}", data);

    let mut conn = env.redis().await?;
    conn.set("name", "tang12").await?;
    let name: String = conn.get("name").await?;
    println!("{:?}", name);

    Ok(())
}
