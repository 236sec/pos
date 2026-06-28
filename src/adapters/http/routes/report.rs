use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use serde_json::{json, Value};
use tracing::instrument;

use crate::{
    adapters::http::app_state::AppState,
    application::use_cases::report::ReportUseCases,
};

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health))
}

#[instrument(skip(_report_use_cases))]
async fn health(
    State(_report_use_cases): State<Arc<ReportUseCases>>,
) -> Json<Value> {
    Json(json!({ "service": "report", "status": "ok" }))
}
