use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Authorization denied: {0}")]
    AuthorizationError(String),
}

pub type AppResult<T> = Result<T, AppError>;
