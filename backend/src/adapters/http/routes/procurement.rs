use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use serde_json::{Value, json};
use tracing::instrument;

use crate::{
    adapters::http::app_state::AppState, application::use_cases::procurement::ProcurementUseCases,
};

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health))
}

#[instrument(skip(_procurement_use_cases))]
async fn health(State(_procurement_use_cases): State<Arc<ProcurementUseCases>>) -> Json<Value> {
    Json(json!({ "service": "procurement", "status": "ok" }))
}
