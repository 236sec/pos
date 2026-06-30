use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post, put},
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    adapters::http::app_state::AppState,
    application::use_cases::floor_plan::FloorPlanUseCases,
    domain::entities::floor_plan::{CreateReservationRequest, Table, UpdateTableStatusRequest},
};

#[derive(Debug, Deserialize)]
struct TableQuery {
    zone_id: Option<Uuid>,
    status: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
struct ZoneGroup {
    id: Uuid,
    name: String,
    floor: i32,
    tables: Vec<Table>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_tables))
        .route("/{id}", get(get_table))
        .route("/{id}/status", put(update_status))
        .route("/{id}/reserve", post(reserve))
}

#[instrument(skip(state))]
async fn list_tables(
    State(state): State<Arc<FloorPlanUseCases>>,
    Query(query): Query<TableQuery>,
) -> Result<Json<Value>, crate::application::app_error::AppError> {
    let tables = state.list_tables(query.zone_id, query.status).await?;

    // Group into zones
    let mut zones: Vec<ZoneGroup> = Vec::new();
    for table in &tables {
        let zone_idx = zones.iter().position(|z| z.id == table.zone_id);
        if let Some(idx) = zone_idx {
            zones[idx].tables.push(Table {
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
            });
        } else {
            zones.push(ZoneGroup {
                id: table.zone_id,
                name: table.zone_name.clone(),
                floor: 0, // floor info not in the join; could add via a zone query
                tables: vec![Table {
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
                }],
            });
        }
    }

    Ok(Json(json!({ "data": zones })))
}

#[instrument(skip(state))]
async fn get_table(
    State(state): State<Arc<FloorPlanUseCases>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, crate::application::app_error::AppError> {
    let table = state.get_table(id).await?;
    Ok(Json(json!({ "data": table })))
}

#[instrument(skip(state))]
async fn update_status(
    State(state): State<Arc<FloorPlanUseCases>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTableStatusRequest>,
) -> Result<Json<Value>, crate::application::app_error::AppError> {
    let table = state.update_status(id, &body.status).await?;
    Ok(Json(json!({ "data": table })))
}

#[instrument(skip(state))]
async fn reserve(
    State(state): State<Arc<FloorPlanUseCases>>,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateReservationRequest>,
) -> Result<Json<Value>, crate::application::app_error::AppError> {
    let start_time =
        NaiveDateTime::parse_from_str(&body.start_time, "%Y-%m-%dT%H:%M:%S").map_err(|e| {
            crate::application::app_error::AppError::Internal(format!(
                "Invalid start_time format: {}. Expected ISO 8601 (YYYY-MM-DDTHH:MM:SS)",
                e
            ))
        })?;
    let end_time =
        NaiveDateTime::parse_from_str(&body.end_time, "%Y-%m-%dT%H:%M:%S").map_err(|e| {
            crate::application::app_error::AppError::Internal(format!(
                "Invalid end_time format: {}. Expected ISO 8601 (YYYY-MM-DDTHH:MM:SS)",
                e
            ))
        })?;

    let reservation = state
        .reserve_table(id, &body.customer_name, start_time, end_time)
        .await?;

    Ok(Json(json!({ "data": reservation })))
}
