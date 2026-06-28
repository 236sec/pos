use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};

use crate::application::app_error::AppResult;

#[async_trait]
pub trait ReportPersistence: Send + Sync {
    async fn get_daily(&self, date: &chrono::NaiveDate) -> AppResult<()>;
}

#[derive(Clone)]
pub struct ReportUseCases {
    persistence: Arc<dyn ReportPersistence>,
}

impl ReportUseCases {
    pub fn new(persistence: Arc<dyn ReportPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn daily_summary(&self, date: &chrono::NaiveDate) -> AppResult<()> {
        info!("Fetching daily summary...");

        self.persistence.get_daily(date).await?;

        info!("Fetching daily summary finished.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    struct MockReportPersistence;

    #[async_trait]
    impl ReportPersistence for MockReportPersistence {
        async fn get_daily(&self, _date: &chrono::NaiveDate) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn daily_summary_works() {
        let use_cases = ReportUseCases::new(Arc::new(MockReportPersistence));
        let date = chrono::NaiveDate::from_ymd_opt(2026, 6, 28).unwrap();

        let result = use_cases.daily_summary(&date).await;

        assert!(result.is_ok());
    }
}
