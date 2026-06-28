use sqlx::PgPool;

use crate::application::app_error::AppError;

pub mod auth;
pub mod inventory;
pub mod menu;
pub mod notification;
pub mod pos;
pub mod procurement;
pub mod report;

#[derive(Clone)]
pub struct PostgresPersistence {
    pub pool: PgPool,
}

impl PostgresPersistence {
    pub fn new(pool: PgPool) -> Self {
        PostgresPersistence { pool }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::Database(value.to_string())
    }
}
