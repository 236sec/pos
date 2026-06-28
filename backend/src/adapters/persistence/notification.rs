use async_trait::async_trait;

use crate::{
    adapters::persistence::PostgresPersistence,
    application::{app_error::AppResult, use_cases::notification::NotificationPersistence},
};

#[async_trait]
impl NotificationPersistence for PostgresPersistence {
    async fn send(&self, _message: &str) -> AppResult<()> {
        todo!("Implement send notification persistence")
    }
}
