use async_trait::async_trait;

use crate::{
    adapters::persistence::PostgresPersistence,
    application::{app_error::AppResult, use_cases::menu::MenuPersistence},
};

#[async_trait]
impl MenuPersistence for PostgresPersistence {
    async fn create_item(&self, _name: &str) -> AppResult<()> {
        todo!("Implement create_item persistence")
    }
}
