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
use uuid::Uuid;

use crate::{
    adapters::http::app_state::AppState,
    application::use_cases::auth::{AuthUseCases, LoginRequest, LoginResponse, UserResponse},
    domain::entities::auth::{Permission, Role},
    infra::config::AppConfig,
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
    State(config): State<Arc<AppConfig>>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, crate::application::app_error::AppError> {
    if config.auth_bypass {
        return Ok(Json(mock_login_response()));
    }
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
    State(config): State<Arc<AppConfig>>,
    bearer: BearerToken,
) -> Result<Json<UserResponse>, crate::application::app_error::AppError> {
    if config.auth_bypass {
        return Ok(Json(mock_user_response()));
    }
    let response = state.me(&bearer.token).await?;
    Ok(Json(response))
}

// ── Bearer token extractor ────────────────────────────────────────────────

pub struct BearerToken {
    pub token: String,
}

// ── Bypass mock helpers ──────────────────────────────────────────────────

const MOCK_TOKEN: &str = "bypass-mock-token";

fn mock_user_response() -> UserResponse {
    UserResponse {
        id: Uuid::nil(),
        username: "dev-user".into(),
        email: "dev@pos.local".into(),
        roles: vec![Role {
            id: Uuid::nil(),
            name: "admin".into(),
        }],
        permissions: vec![Permission {
            id: Uuid::nil(),
            resource: "*".into(),
            action: "*".into(),
        }],
    }
}

fn mock_login_response() -> LoginResponse {
    LoginResponse {
        token: MOCK_TOKEN.into(),
        user: mock_user_response(),
    }
}

impl<S> FromRequestParts<S> for BearerToken
where
    S: Send + Sync,
    Arc<AuthUseCases>: FromRef<S>,
    Arc<AppConfig>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = Arc::<AppConfig>::from_ref(state);
        if config.auth_bypass {
            return Ok(BearerToken {
                token: MOCK_TOKEN.into(),
            });
        }

        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?;

        let token = auth_header.strip_prefix("Bearer ").ok_or((
            StatusCode::UNAUTHORIZED,
            "Invalid Authorization header format",
        ))?;

        Ok(BearerToken {
            token: token.to_string(),
        })
    }
}
