use async_trait::async_trait;

use crate::{
    adapters::persistence::PostgresPersistence,
    application::{app_error::AppResult, use_cases::procurement::ProcurementPersistence},
};

#[async_trait]
impl ProcurementPersistence for PostgresPersistence {
    async fn create_po(&self, _supplier_id: &uuid::Uuid, _status: &str) -> AppResult<()> {
        todo!("Implement create_po persistence")
    }
}
