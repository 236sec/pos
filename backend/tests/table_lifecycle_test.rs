use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;

use pos::application::app_error::AppResult;
use pos::application::use_cases::floor_plan::{
    FloorPlanPersistence, FloorPlanUseCases,
};
use pos::domain::entities::floor_plan::{Reservation, Table, TableWithZone};

struct MockFloorPlanPersistence {
    tables: std::sync::Mutex<Vec<TableWithZone>>,
}

impl MockFloorPlanPersistence {
    fn new() -> Self {
        Self {
            tables: std::sync::Mutex::new(vec![]),
        }
    }
}

#[async_trait]
impl FloorPlanPersistence for MockFloorPlanPersistence {
    async fn list_tables(
        &self,
        zone_id: Option<Uuid>,
        status: Option<String>,
    ) -> AppResult<Vec<TableWithZone>> {
        let tables = self.tables.lock().unwrap();
        Ok(tables
            .iter()
            .filter(|t| {
                let match_zone = zone_id.map(|z| t.zone_id == z).unwrap_or(true);
                let match_status = status
                    .as_ref()
                    .map(|s| t.status == *s)
                    .unwrap_or(true);
                match_zone && match_status
            })
            .cloned()
            .collect())
    }

    async fn get_table(&self, id: Uuid) -> AppResult<Option<TableWithZone>> {
        let tables = self.tables.lock().unwrap();
        Ok(tables.iter().find(|t| t.id == id).cloned())
    }

    async fn update_table_status(&self, id: Uuid, new_status: &str) -> AppResult<Table> {
        let mut tables = self.tables.lock().unwrap();
        if let Some(table) = tables.iter_mut().find(|t| t.id == id) {
            table.status = new_status.to_string();
            Ok(Table {
                id: table.id,
                name: table.name.clone(),
                zone_id: table.zone_id,
                zone_name: Some(table.zone_name.clone()),
                x: table.x,
                y: table.y,
                seats: table.seats,
                status: table.status.clone(),
                current_order_id: table.current_order_id,
                created_at: table.created_at,
            })
        } else {
            Err(pos::application::app_error::AppError::NotFound(
                "Table not found".to_string(),
            ))
        }
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
async fn table_lifecycle_works() {
    let persistence = Arc::new(MockFloorPlanPersistence::new());
    let use_cases = FloorPlanUseCases::new(persistence.clone());

    let zone_id = Uuid::new_v4();
    let table_id = Uuid::new_v4();

    // Seed a table in "available" status
    {
        let mut tables = persistence.tables.lock().unwrap();
        tables.push(TableWithZone {
            id: table_id,
            name: "Table 1".to_string(),
            zone_id,
            zone_name: "Main Floor".to_string(),
            x: 100.0,
            y: 200.0,
            seats: 4,
            status: "available".to_string(),
            current_order_id: None,
            created_at: chrono::Utc::now().naive_utc(),
        });
    }

    // 1. List tables — should be 1
    let tables = use_cases.list_tables(None, None).await.unwrap();
    assert_eq!(tables.len(), 1);
    assert_eq!(tables[0].status, "available");

    // 2. Update to occupied
    let updated = use_cases.update_status(table_id, "occupied").await.unwrap();
    assert_eq!(updated.status, "occupied");

    // 3. Update to dirty
    let updated = use_cases.update_status(table_id, "dirty").await.unwrap();
    assert_eq!(updated.status, "dirty");

    // 4. Update back to available
    let updated = use_cases
        .update_status(table_id, "available")
        .await
        .unwrap();
    assert_eq!(updated.status, "available");

    // 5. Invalid transition: available -> dirty should fail
    let result = use_cases.update_status(table_id, "dirty").await;
    assert!(result.is_err());

    // 6. Reserve table
    let reservation = use_cases
        .reserve_table(
            table_id,
            "Jane Doe",
            NaiveDateTime::parse_from_str("2026-07-01 18:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2026-07-01 20:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(reservation.customer_name, "Jane Doe");
    assert_eq!(reservation.table_id, table_id);
}
