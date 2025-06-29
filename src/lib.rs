pub mod api;
pub mod config;
pub mod db;
pub mod domain;
pub mod errors;
pub mod services;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::load()?;
    let db_pool = db::init_pg::init(&cfg.db).await.unwrap();
    api::start_server(cfg, db_pool).await?
}
