use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};

use crate::application::app_error::AppResult;

#[async_trait]
pub trait InventoryPersistence: Send + Sync {
    async fn adjust_stock(&self, ingredient_name: &str, delta: i64) -> AppResult<()>;
}

#[derive(Clone)]
pub struct InventoryUseCases {
    persistence: Arc<dyn InventoryPersistence>,
}

impl InventoryUseCases {
    pub fn new(persistence: Arc<dyn InventoryPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn adjust(&self, ingredient_name: &str, delta: i64) -> AppResult<()> {
        info!("Adjusting inventory...");

        self.persistence
            .adjust_stock(ingredient_name, delta)
            .await?;

        info!("Adjusting inventory finished.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    struct MockInventoryPersistence;

    #[async_trait]
    impl InventoryPersistence for MockInventoryPersistence {
        async fn adjust_stock(&self, _ingredient_name: &str, _delta: i64) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn adjust_works() {
        let use_cases = InventoryUseCases::new(Arc::new(MockInventoryPersistence));

        let result = use_cases.adjust("Coffee beans", -100).await;

        assert!(result.is_ok());
    }
}
