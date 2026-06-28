use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use serde_json::{Value, json};
use tracing::instrument;

use crate::{adapters::http::app_state::AppState, application::use_cases::auth::AuthUseCases};

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health))
}

#[instrument(skip(_auth_use_cases))]
async fn health(State(_auth_use_cases): State<Arc<AuthUseCases>>) -> Json<Value> {
    Json(json!({ "service": "auth", "status": "ok" }))
}
