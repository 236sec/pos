use std::sync::Arc;

use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use tracing::{info, instrument};

use crate::application::app_error::AppResult;

#[async_trait]
pub trait AuthPersistence: Send + Sync {
    async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> AppResult<()>;
}

#[derive(Clone)]
pub struct AuthUseCases {
    persistence: Arc<dyn AuthPersistence>,
}

impl AuthUseCases {
    pub fn new(persistence: Arc<dyn AuthPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn add(&self, username: &str, email: &str, password: &SecretString) -> AppResult<()> {
        info!("Adding user...");

        self.persistence
            .create_user(username, email, password.expose_secret())
            .await?;

        info!("Adding user finished.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    struct MockAuthPersistence;

    #[async_trait]
    impl AuthPersistence for MockAuthPersistence {
        async fn create_user(
            &self,
            _username: &str,
            _email: &str,
            _password_hash: &str,
        ) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn add_user_works() {
        let use_cases = AuthUseCases::new(Arc::new(MockAuthPersistence));

        let result = use_cases
            .add("testuser", "testuser@example.com", &"test_pw".into())
            .await;

        assert!(result.is_ok());
    }
}
