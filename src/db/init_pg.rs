use crate::{config::Database, db::user_pg::PostgresUserRepo};
use sqlx::postgres::PgPoolOptions;
use std::error::Error;

pub async fn init(db_config: &Database) -> Result<PostgresUserRepo, Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_config.url) //"postgres://postgres:password@localhost/test"
        .await
        .unwrap();
    let user_repo = PostgresUserRepo::new(pool);
    Ok(user_repo)
}
