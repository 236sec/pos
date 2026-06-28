use async_trait::async_trait;

use crate::{
    adapters::persistence::PostgresPersistence,
    application::{app_error::AppResult, use_cases::inventory::InventoryPersistence},
};

#[async_trait]
impl InventoryPersistence for PostgresPersistence {
    async fn adjust_stock(&self, _ingredient_name: &str, _delta: i64) -> AppResult<()> {
        todo!("Implement adjust_stock persistence")
    }
}
