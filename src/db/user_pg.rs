// src/db/user_pg.rs
use crate::domain::user::{User, UserRepo};
use crate::errors::AppError;
use sqlx::PgPool;

pub struct PostgresUserRepo {
    pool: PgPool,
}

impl PostgresUserRepo {
    pub fn new(pool: PgPool) -> PostgresUserRepo {
        PostgresUserRepo { pool }
    }
}

impl UserRepo for PostgresUserRepo {
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<User, AppError> {
        let rec = sqlx::query_as!(
            User,
            "SELECT id, email, is_active FROM users WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(rec)
    }
}
