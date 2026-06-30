use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TableZone {
    pub id: Uuid,
    pub name: String,
    pub floor: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Table {
    pub id: Uuid,
    pub name: String,
    pub zone_id: Uuid,
    pub zone_name: Option<String>,
    pub x: f64,
    pub y: f64,
    pub seats: i32,
    pub status: String,
    pub current_order_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TableWithZone {
    pub id: Uuid,
    pub name: String,
    pub zone_id: Uuid,
    pub zone_name: String,
    pub x: f64,
    pub y: f64,
    pub seats: i32,
    pub status: String,
    pub current_order_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Reservation {
    pub id: Uuid,
    pub table_id: Uuid,
    pub customer_name: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTableRequest {
    pub name: String,
    pub zone_id: Uuid,
    pub x: f64,
    pub y: f64,
    pub seats: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTableStatusRequest {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReservationRequest {
    pub customer_name: String,
    pub start_time: String,
    pub end_time: String,
}
