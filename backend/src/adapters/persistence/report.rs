use async_trait::async_trait;

use crate::{
    adapters::persistence::PostgresPersistence,
    application::{app_error::AppResult, use_cases::report::ReportPersistence},
};

#[async_trait]
impl ReportPersistence for PostgresPersistence {
    async fn get_daily(&self, _date: &chrono::NaiveDate) -> AppResult<()> {
        todo!("Implement get_daily persistence")
    }
}
