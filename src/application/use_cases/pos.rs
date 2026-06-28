use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};

use crate::application::app_error::AppResult;

#[async_trait]
pub trait PosPersistence: Send + Sync {
    async fn create_order(&self, branch_id: &uuid::Uuid, status: &str) -> AppResult<()>;
}

#[derive(Clone)]
pub struct PosUseCases {
    persistence: Arc<dyn PosPersistence>,
}

impl PosUseCases {
    pub fn new(persistence: Arc<dyn PosPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn add_order(&self, branch_id: &uuid::Uuid, status: &str) -> AppResult<()> {
        info!("Creating order...");

        self.persistence.create_order(branch_id, status).await?;

        info!("Creating order finished.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    struct MockPosPersistence;

    #[async_trait]
    impl PosPersistence for MockPosPersistence {
        async fn create_order(&self, _branch_id: &uuid::Uuid, _status: &str) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn add_order_works() {
        let use_cases = PosUseCases::new(Arc::new(MockPosPersistence));
        let branch_id = uuid::Uuid::new_v4();

        let result = use_cases.add_order(&branch_id, "pending").await;

        assert!(result.is_ok());
    }
}
