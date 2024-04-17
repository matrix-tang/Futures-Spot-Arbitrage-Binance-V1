use crate::conf;
use anyhow::anyhow;
use once_cell::sync::OnceCell;
use rocksdb::{DBWithThreadMode, MultiThreaded, DB};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::fs::remove_dir_all;

#[derive(Debug, Clone)]
pub struct Db {
    db_pool: MySqlPool,
    redis: redis::Client,
    rocksdb: Arc<DBWithThreadMode<MultiThreaded>>,
}

pub static DBV1: OnceCell<Db> = OnceCell::new();

pub async fn init_env() -> anyhow::Result<()> {
    let db = Db::new().await?;
    if let Err(e) = DBV1.set(db) {
        panic!("{:?}", e);
    }
    Ok(())
}

pub fn get_db<'a>() -> anyhow::Result<&'a Db> {
    match DBV1.get() {
        Some(db) => Ok(db),
        None => Err(anyhow!("db is none.")),
    }
}

impl Db {
    pub async fn new() -> anyhow::Result<Self> {
        let db_pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(conf::C.mysql.url.as_str())
            .await?;
        let redis = redis::Client::open(conf::C.redis.url.as_str())?;

        remove_dir_all(&conf::C.rocksdb.path).await?;
        let rocksdb = Arc::new(DB::open_default(&conf::C.rocksdb.path).unwrap());
        Ok(Self {
            db_pool,
            redis,
            rocksdb,
        })
    }

    pub fn rocksdb(&self) -> &Arc<DBWithThreadMode<MultiThreaded>> {
        &self.rocksdb
    }

    pub fn database(&self) -> &MySqlPool {
        &self.db_pool
    }

    pub async fn redis(&self) -> anyhow::Result<redis::aio::MultiplexedConnection> {
        self.redis
            .get_multiplexed_tokio_connection()
            .await
            .map_err(|e| e.into())
    }
}
