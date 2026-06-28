use async_trait::async_trait;

use crate::{
    adapters::persistence::PostgresPersistence,
    application::{app_error::AppResult, use_cases::pos::PosPersistence},
};

#[async_trait]
impl PosPersistence for PostgresPersistence {
    async fn create_order(&self, _branch_id: &uuid::Uuid, _status: &str) -> AppResult<()> {
        todo!("Implement create_order persistence")
    }
}
