use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    application::{app_error::AppResult, use_cases::auth::AuthPersistence},
};

#[async_trait]
impl AuthPersistence for PostgresPersistence {
    async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> AppResult<()> {
        let uuid = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)",
        )
        .bind(uuid)
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
