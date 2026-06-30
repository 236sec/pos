pub mod auth;
pub mod floor_plan;
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
        .nest("/auth", auth::router())
        .nest("/pos", pos::router())
        .nest("/menu", menu::router())
        .nest("/inventory", inventory::router())
        .nest("/procurement", procurement::router())
        .nest("/report", report::router())
        .nest("/notification", notification::router())
        .nest("/tables", floor_plan::router())
}
