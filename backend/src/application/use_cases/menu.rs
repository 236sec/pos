use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};

use crate::application::app_error::AppResult;

#[async_trait]
pub trait MenuPersistence: Send + Sync {
    async fn create_item(&self, name: &str) -> AppResult<()>;
}

#[derive(Clone)]
pub struct MenuUseCases {
    persistence: Arc<dyn MenuPersistence>,
}

impl MenuUseCases {
    pub fn new(persistence: Arc<dyn MenuPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn add_item(&self, name: &str) -> AppResult<()> {
        info!("Adding menu item...");

        self.persistence.create_item(name).await?;

        info!("Adding menu item finished.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    struct MockMenuPersistence;

    #[async_trait]
    impl MenuPersistence for MockMenuPersistence {
        async fn create_item(&self, _name: &str) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn add_item_works() {
        let use_cases = MenuUseCases::new(Arc::new(MockMenuPersistence));

        let result = use_cases.add_item("Espresso").await;

        assert!(result.is_ok());
    }
}
