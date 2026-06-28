pub mod auth;
pub mod inventory;
pub mod menu;
pub mod notification;
pub mod pos;
pub mod procurement;
pub mod report;

use axum::Router;

use crate::adapters::http::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/api/auth", auth::router())
        .nest("/api/pos", pos::router())
        .nest("/api/menu", menu::router())
        .nest("/api/inventory", inventory::router())
        .nest("/api/procurement", procurement::router())
        .nest("/api/report", report::router())
        .nest("/api/notification", notification::router())
}
