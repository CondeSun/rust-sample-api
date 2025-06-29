// src/services/user.rs
use crate::domain::user::{User, UserRepo};
use crate::errors::AppError;

#[derive(Clone)]
pub struct UserService<R: UserRepo> {
    repo: R,
}

impl<R: UserRepo> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
    pub async fn get_user(&self, id: uuid::Uuid) -> Result<User, AppError> {
        let user = self.repo.find_by_id(id).await?;
        if !user.is_active {
            Err(AppError::InactiveUser)
        } else {
            Ok(user)
        }
    }
}
