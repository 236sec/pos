use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};

use crate::application::app_error::AppResult;

#[async_trait]
pub trait NotificationPersistence: Send + Sync {
    async fn send(&self, message: &str) -> AppResult<()>;
}

#[derive(Clone)]
pub struct NotificationUseCases {
    persistence: Arc<dyn NotificationPersistence>,
}

impl NotificationUseCases {
    pub fn new(persistence: Arc<dyn NotificationPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn dispatch(&self, message: &str) -> AppResult<()> {
        info!("Dispatching notification...");

        self.persistence.send(message).await?;

        info!("Dispatching notification finished.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    struct MockNotificationPersistence;

    #[async_trait]
    impl NotificationPersistence for MockNotificationPersistence {
        async fn send(&self, _message: &str) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn dispatch_works() {
        let use_cases = NotificationUseCases::new(Arc::new(MockNotificationPersistence));

        let result = use_cases.dispatch("Order ready for pickup").await;

        assert!(result.is_ok());
    }
}
