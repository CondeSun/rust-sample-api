// src/domain/user.rs
#[derive(Debug, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub is_active: bool,
}

pub trait UserRepo {
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<User, crate::errors::AppError>;
}
