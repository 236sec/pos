use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use serde_json::{json, Value};
use tracing::instrument;

use crate::{
    adapters::http::app_state::AppState,
    application::use_cases::inventory::InventoryUseCases,
};

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health))
}

#[instrument(skip(_inventory_use_cases))]
async fn health(
    State(_inventory_use_cases): State<Arc<InventoryUseCases>>,
) -> Json<Value> {
    Json(json!({ "service": "inventory", "status": "ok" }))
}
