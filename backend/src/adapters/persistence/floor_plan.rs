use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    application::{app_error::AppResult, use_cases::floor_plan::FloorPlanPersistence},
    domain::entities::floor_plan::{Reservation, Table, TableWithZone},
};

#[async_trait]
impl FloorPlanPersistence for PostgresPersistence {
    async fn list_tables(
        &self,
        zone_id: Option<Uuid>,
        status: Option<String>,
    ) -> AppResult<Vec<TableWithZone>> {
        let mut query = sqlx::QueryBuilder::new(
            "SELECT t.id, t.name, t.zone_id, tz.name as zone_name, t.x, t.y, t.seats, t.status, t.current_order_id, t.created_at FROM tables t JOIN table_zones tz ON t.zone_id = tz.id WHERE 1=1",
        );

        if let Some(ref zone_id) = zone_id {
            query.push(" AND t.zone_id = ");
            query.push_bind(zone_id);
        }

        if let Some(ref status) = status {
            query.push(" AND t.status = ");
            query.push_bind(status);
        }

        query.push(" ORDER BY tz.floor, tz.name, t.name");

        let tables = query
            .build_query_as::<TableWithZone>()
            .fetch_all(&self.pool)
            .await?;

        Ok(tables)
    }

    async fn get_table(&self, id: Uuid) -> AppResult<Option<TableWithZone>> {
        let table = sqlx::query_as::<_, TableWithZone>(
            "SELECT t.id, t.name, t.zone_id, tz.name as zone_name, t.x, t.y, t.seats, t.status, t.current_order_id, t.created_at \
             FROM tables t \
             JOIN table_zones tz ON t.zone_id = tz.id \
             WHERE t.id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(table)
    }

    async fn update_table_status(&self, id: Uuid, new_status: &str) -> AppResult<Table> {
        let rows_affected = sqlx::query("UPDATE tables SET status = $1 WHERE id = $2")
            .bind(new_status)
            .bind(id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(crate::application::app_error::AppError::NotFound(format!(
                "Table {} not found",
                id
            )));
        }

        // Secondary query to get zone_name via JOIN
        let table = sqlx::query_as::<_, Table>(
            "SELECT t.*, tz.name as zone_name FROM tables t \
             JOIN table_zones tz ON t.zone_id = tz.id \
             WHERE t.id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| {
            crate::application::app_error::AppError::NotFound(format!("Table {} not found", id))
        })?;

        Ok(table)
    }

    async fn create_reservation(
        &self,
        table_id: Uuid,
        customer_name: &str,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> AppResult<Reservation> {
        let reservation = sqlx::query_as::<_, Reservation>(
            "INSERT INTO reservations (table_id, customer_name, start_time, end_time) \
             VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(table_id)
        .bind(customer_name)
        .bind(start_time)
        .bind(end_time)
        .fetch_one(&self.pool)
        .await?;

        Ok(reservation)
    }
}
