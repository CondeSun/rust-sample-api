// src/config/mod.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db: Database,
    pub server: Server,
}
#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}
#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
}
pub fn load() -> Result<Config, config::ConfigError> {
    let cfg = config::Config::builder()
        .add_source(config::File::with_name("config/default"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;
    cfg.try_deserialize()
}
