use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{FromRef, FromRequestParts, State},
    http::{StatusCode, request::Parts},
    response::IntoResponse,
    routing::{get, post},
};
use secrecy::SecretString;
use serde_json::{Value, json};
use tracing::instrument;

use crate::{
    adapters::http::app_state::AppState,
    application::use_cases::auth::{AuthUseCases, LoginRequest, LoginResponse, UserResponse},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
}

#[instrument(skip(_auth_use_cases))]
async fn health(State(_auth_use_cases): State<Arc<AuthUseCases>>) -> Json<Value> {
    Json(json!({ "service": "auth", "status": "ok" }))
}

#[instrument(skip(state, body))]
async fn login(
    State(state): State<Arc<AuthUseCases>>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, crate::application::app_error::AppError> {
    let password = SecretString::new(body.password.into());
    let response = state.login(&body.username, &password).await?;
    Ok(Json(response))
}

async fn logout() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

#[instrument(skip(state, bearer))]
async fn me(
    State(state): State<Arc<AuthUseCases>>,
    bearer: BearerToken,
) -> Result<Json<UserResponse>, crate::application::app_error::AppError> {
    let response = state.me(&bearer.token).await?;
    Ok(Json(response))
}

// ── Bearer token extractor ────────────────────────────────────────────────

pub struct BearerToken {
    pub token: String,
}

impl<S> FromRequestParts<S> for BearerToken
where
    S: Send + Sync,
    Arc<AuthUseCases>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid Authorization header format"))?;

        Ok(BearerToken {
            token: token.to_string(),
        })
    }
}
