use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    application::app_error::{AppError, AppResult},
    domain::entities::floor_plan::{Reservation, Table, TableWithZone},
};

#[async_trait]
pub trait FloorPlanPersistence: Send + Sync {
    async fn list_tables(
        &self,
        zone_id: Option<Uuid>,
        status: Option<String>,
    ) -> AppResult<Vec<TableWithZone>>;
    async fn get_table(&self, id: Uuid) -> AppResult<Option<TableWithZone>>;
    async fn update_table_status(&self, id: Uuid, new_status: &str) -> AppResult<Table>;
    async fn create_reservation(
        &self,
        table_id: Uuid,
        customer_name: &str,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> AppResult<Reservation>;
}

#[derive(Clone)]
pub struct FloorPlanUseCases {
    persistence: Arc<dyn FloorPlanPersistence>,
}

impl FloorPlanUseCases {
    pub fn new(persistence: Arc<dyn FloorPlanPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn list_tables(
        &self,
        zone_id: Option<Uuid>,
        status: Option<String>,
    ) -> AppResult<Vec<TableWithZone>> {
        info!("Listing tables...");

        let tables = self.persistence.list_tables(zone_id, status).await?;

        info!("Listing tables finished.");

        Ok(tables)
    }

    #[instrument(skip(self))]
    pub async fn get_table(&self, id: Uuid) -> AppResult<TableWithZone> {
        info!("Getting table...");

        let table = self
            .persistence
            .get_table(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Table {} not found", id)))?;

        info!("Getting table finished.");

        Ok(table)
    }

    #[instrument(skip(self))]
    pub async fn update_status(&self, id: Uuid, new_status: &str) -> AppResult<Table> {
        info!("Updating table status...");

        // Validate state machine transitions
        let table = self
            .persistence
            .get_table(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Table {} not found", id)))?;

        let allowed = match (table.status.as_str(), new_status) {
            ("available", "occupied") => true,
            ("available", "reserved") => true,
            ("occupied", "dirty") => true,
            ("dirty", "available") => true,
            _ => false,
        };

        if !allowed {
            return Err(AppError::Internal(format!(
                "Invalid status transition: {} -> {}",
                table.status, new_status
            )));
        }

        let updated = self.persistence.update_table_status(id, new_status).await?;

        info!("Updating table status finished.");

        Ok(updated)
    }

    #[instrument(skip(self))]
    pub async fn reserve_table(
        &self,
        id: Uuid,
        customer_name: &str,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> AppResult<Reservation> {
        info!("Reserving table...");

        // Verify table exists
        self.get_table(id).await?;

        let reservation = self
            .persistence
            .create_reservation(id, customer_name, start_time, end_time)
            .await?;

        info!("Reserving table finished.");

        Ok(reservation)
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    struct MockFloorPlanPersistence;

    #[async_trait]
    impl FloorPlanPersistence for MockFloorPlanPersistence {
        async fn list_tables(
            &self,
            _zone_id: Option<Uuid>,
            _status: Option<String>,
        ) -> AppResult<Vec<TableWithZone>> {
            Ok(vec![])
        }

        async fn get_table(&self, id: Uuid) -> AppResult<Option<TableWithZone>> {
            Ok(Some(TableWithZone {
                id,
                name: "Test Table".to_string(),
                zone_id: Uuid::new_v4(),
                zone_name: "Test Zone".to_string(),
                x: 0.0,
                y: 0.0,
                seats: 4,
                status: "available".to_string(),
                current_order_id: None,
                created_at: chrono::Utc::now().naive_utc(),
            }))
        }

        async fn update_table_status(
            &self,
            id: Uuid,
            _new_status: &str,
        ) -> AppResult<Table> {
            Ok(Table {
                id,
                name: "Test Table".to_string(),
                zone_id: Uuid::new_v4(),
                zone_name: Some("Test Zone".to_string()),
                x: 0.0,
                y: 0.0,
                seats: 4,
                status: "dirty".to_string(),
                current_order_id: None,
                created_at: chrono::Utc::now().naive_utc(),
            })
        }

        async fn create_reservation(
            &self,
            table_id: Uuid,
            customer_name: &str,
            start_time: NaiveDateTime,
            end_time: NaiveDateTime,
        ) -> AppResult<Reservation> {
            Ok(Reservation {
                id: Uuid::new_v4(),
                table_id,
                customer_name: customer_name.to_string(),
                start_time,
                end_time,
                created_at: chrono::Utc::now().naive_utc(),
            })
        }
    }

    #[tokio::test]
    async fn list_tables_works() {
        let use_cases = FloorPlanUseCases::new(Arc::new(MockFloorPlanPersistence));

        let result = use_cases.list_tables(None, None).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_table_works() {
        let use_cases = FloorPlanUseCases::new(Arc::new(MockFloorPlanPersistence));
        let id = Uuid::new_v4();

        let result = use_cases.get_table(id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_table_not_found() {
        struct MockNotFound;

        #[async_trait]
        impl FloorPlanPersistence for MockNotFound {
            async fn list_tables(
                &self,
                _zone_id: Option<Uuid>,
                _status: Option<String>,
            ) -> AppResult<Vec<TableWithZone>> {
                Ok(vec![])
            }

            async fn get_table(&self, _id: Uuid) -> AppResult<Option<TableWithZone>> {
                Ok(None)
            }

            async fn update_table_status(
                &self,
                _id: Uuid,
                _new_status: &str,
            ) -> AppResult<Table> {
                unreachable!()
            }

            async fn create_reservation(
                &self,
                _table_id: Uuid,
                _customer_name: &str,
                _start_time: NaiveDateTime,
                _end_time: NaiveDateTime,
            ) -> AppResult<Reservation> {
                unreachable!()
            }
        }

        let use_cases = FloorPlanUseCases::new(Arc::new(MockNotFound));

        let result = use_cases.get_table(Uuid::new_v4()).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
    }

    #[tokio::test]
    async fn update_status_valid_transition() {
        let use_cases = FloorPlanUseCases::new(Arc::new(MockFloorPlanPersistence));

        let result = use_cases.update_status(Uuid::new_v4(), "occupied").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_status_invalid_transition() {
        struct MockAvailable;

        #[async_trait]
        impl FloorPlanPersistence for MockAvailable {
            async fn list_tables(
                &self,
                _zone_id: Option<Uuid>,
                _status: Option<String>,
            ) -> AppResult<Vec<TableWithZone>> {
                Ok(vec![])
            }

            async fn get_table(&self, _id: Uuid) -> AppResult<Option<TableWithZone>> {
                Ok(Some(TableWithZone {
                    id: Uuid::new_v4(),
                    name: "Test".to_string(),
                    zone_id: Uuid::new_v4(),
                    zone_name: "Zone".to_string(),
                    x: 0.0,
                    y: 0.0,
                    seats: 4,
                    status: "available".to_string(),
                    current_order_id: None,
                    created_at: chrono::Utc::now().naive_utc(),
                }))
            }

            async fn update_table_status(
                &self,
                _id: Uuid,
                _new_status: &str,
            ) -> AppResult<Table> {
                unreachable!()
            }

            async fn create_reservation(
                &self,
                _table_id: Uuid,
                _customer_name: &str,
                _start_time: NaiveDateTime,
                _end_time: NaiveDateTime,
            ) -> AppResult<Reservation> {
                unreachable!()
            }
        }

        let use_cases = FloorPlanUseCases::new(Arc::new(MockAvailable));

        let result = use_cases.update_status(Uuid::new_v4(), "dirty").await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Internal(_)));
    }

    #[tokio::test]
    async fn reserve_table_works() {
        let use_cases = FloorPlanUseCases::new(Arc::new(MockFloorPlanPersistence));

        let result = use_cases
            .reserve_table(
                Uuid::new_v4(),
                "John Doe",
                NaiveDateTime::parse_from_str("2026-07-01 18:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2026-07-01 20:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            )
            .await;

        assert!(result.is_ok());
    }
}
