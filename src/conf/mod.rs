pub mod redis_key;

use std::fs::File;
use std::io::prelude::*;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LogConfig {
    pub pattern: String,
    pub dir: String,
    pub prefix: String,
    pub level: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MysqlConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BinanceApiConfig {
    pub api_key: String,
    pub secret_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub redis: RedisConfig,
    pub mysql: MysqlConfig,
    pub log: LogConfig,
    pub binance_api_config: BinanceApiConfig,
}

lazy_static! {
    pub static ref C: Config = init_config().unwrap();
}

pub fn init_config() -> anyhow::Result<Config> {
    let config_path = env!("CARGO_MANIFEST_DIR");
    let path = config_path.to_string() + "/config.toml";
    let mut file = File::open(path)?;
    let mut str_val = String::new();
    file.read_to_string(&mut str_val)?;
    let cfg: Config = toml::from_str(&str_val)?;
    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config() {
        let c = init_config().unwrap();
        println!("{:#?}", c.redis.url);
        println!("{:#?}", c.mysql.url);
        println!("{:#?}", c.binance_api_config);
    }
}
