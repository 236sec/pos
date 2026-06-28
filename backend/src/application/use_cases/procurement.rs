use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};

use crate::application::app_error::AppResult;

#[async_trait]
pub trait ProcurementPersistence: Send + Sync {
    async fn create_po(&self, supplier_id: &uuid::Uuid, status: &str) -> AppResult<()>;
}

#[derive(Clone)]
pub struct ProcurementUseCases {
    persistence: Arc<dyn ProcurementPersistence>,
}

impl ProcurementUseCases {
    pub fn new(persistence: Arc<dyn ProcurementPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn add_po(&self, supplier_id: &uuid::Uuid, status: &str) -> AppResult<()> {
        info!("Creating purchase order...");

        self.persistence.create_po(supplier_id, status).await?;

        info!("Creating purchase order finished.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    struct MockProcurementPersistence;

    #[async_trait]
    impl ProcurementPersistence for MockProcurementPersistence {
        async fn create_po(&self, _supplier_id: &uuid::Uuid, _status: &str) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn add_po_works() {
        let use_cases = ProcurementUseCases::new(Arc::new(MockProcurementPersistence));
        let supplier_id = uuid::Uuid::new_v4();

        let result = use_cases.add_po(&supplier_id, "draft").await;

        assert!(result.is_ok());
    }
}
