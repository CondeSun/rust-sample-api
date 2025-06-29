use std::fmt::Display;

use thiserror::Error;

// src/errors/mod.rs
#[derive(Error, Debug)]
pub enum AppError {
    NotFound,
    InactiveUser,
    DbError(#[from] sqlx::Error),
}

/*
 * Replaced by thiserror crate
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::DbError(e)
    }
}
 */

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "this should be an error impl here..."),
        }
    }
}
