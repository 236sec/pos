use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};
use serde_json::{Value, json};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    adapters::http::app_state::AppState,
    application::{
        app_error::AppResult,
        use_cases::menu::{CreateItemInput, MenuUseCases, UpdateItemInput},
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        // Items
        .route("/items", post(create_item))
        .route("/items", get(get_all_items))
        .route("/items/:id", get(get_item_detail))
        .route("/items/:id", put(update_item))
        .route("/items/:id", delete(delete_item))
        // Categories
        .route("/categories", get(get_all_categories))
        .route("/categories", post(create_category))
        .route("/categories/:id", put(update_category))
        .route("/categories/:id", delete(delete_category))
}

#[instrument(skip(_menu_use_cases))]
async fn health(State(_menu_use_cases): State<Arc<MenuUseCases>>) -> Json<Value> {
    Json(json!({ "service": "menu", "status": "ok" }))
}

// ── Items ──

#[instrument(skip(state))]
async fn create_item(
    State(state): State<Arc<MenuUseCases>>,
    Json(body): Json<CreateItemInput>,
) -> AppResult<Json<Value>> {
    let item = state.create_item(&body).await?;
    Ok(Json(json!(item)))
}

#[instrument(skip(state))]
async fn get_all_items(State(state): State<Arc<MenuUseCases>>) -> AppResult<Json<Value>> {
    let items = state.get_all_items().await?;
    Ok(Json(json!(items)))
}

#[instrument(skip(state))]
async fn get_item_detail(
    State(state): State<Arc<MenuUseCases>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    let detail = state.get_item_detail(id).await?;
    Ok(Json(json!(detail)))
}

#[instrument(skip(state))]
async fn update_item(
    State(state): State<Arc<MenuUseCases>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateItemInput>,
) -> AppResult<Json<Value>> {
    let item = state.update_item(id, &body).await?;
    Ok(Json(json!(item)))
}

#[instrument(skip(state))]
async fn delete_item(
    State(state): State<Arc<MenuUseCases>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    state.soft_delete_item(id).await?;
    Ok(Json(json!({ "deleted": true })))
}

// ── Categories ──

#[instrument(skip(state))]
async fn get_all_categories(State(state): State<Arc<MenuUseCases>>) -> AppResult<Json<Value>> {
    let categories = state.get_all_categories().await?;
    Ok(Json(json!(categories)))
}

#[instrument(skip(state))]
async fn create_category(
    State(state): State<Arc<MenuUseCases>>,
    Json(body): Json<CreateCategoryBody>,
) -> AppResult<Json<Value>> {
    let cat = state
        .create_category(&body.name, &body.name_en, body.sort_order)
        .await?;
    Ok(Json(json!(cat)))
}

#[instrument(skip(state))]
async fn update_category(
    State(state): State<Arc<MenuUseCases>>,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateCategoryBody>,
) -> AppResult<Json<Value>> {
    let cat = state
        .update_category(id, &body.name, &body.name_en, body.sort_order)
        .await?;
    Ok(Json(json!(cat)))
}

#[instrument(skip(state))]
async fn delete_category(
    State(state): State<Arc<MenuUseCases>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    state.soft_delete_category(id).await?;
    Ok(Json(json!({ "deleted": true })))
}

// ── Request bodies ────────────────────────────────────────────────────────

#[derive(Debug, serde::Deserialize)]
struct CreateCategoryBody {
    name: String,
    name_en: String,
    sort_order: i32,
}
