use std::sync::Arc;
use axum::extract::FromRef;

use crate::application::use_cases::auth::AuthUseCases;
use crate::application::use_cases::inventory::InventoryUseCases;
use crate::application::use_cases::menu::MenuUseCases;
use crate::application::use_cases::notification::NotificationUseCases;
use crate::application::use_cases::pos::PosUseCases;
use crate::application::use_cases::procurement::ProcurementUseCases;
use crate::application::use_cases::report::ReportUseCases;
use crate::infra::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub auth_use_cases: Arc<AuthUseCases>,
    pub pos_use_cases: Arc<PosUseCases>,
    pub menu_use_cases: Arc<MenuUseCases>,
    pub inventory_use_cases: Arc<InventoryUseCases>,
    pub procurement_use_cases: Arc<ProcurementUseCases>,
    pub report_use_cases: Arc<ReportUseCases>,
    pub notification_use_cases: Arc<NotificationUseCases>,
}

impl FromRef<AppState> for Arc<AuthUseCases> {
    fn from_ref(state: &AppState) -> Self { state.auth_use_cases.clone() }
}

impl FromRef<AppState> for Arc<PosUseCases> {
    fn from_ref(state: &AppState) -> Self { state.pos_use_cases.clone() }
}

impl FromRef<AppState> for Arc<MenuUseCases> {
    fn from_ref(state: &AppState) -> Self { state.menu_use_cases.clone() }
}

impl FromRef<AppState> for Arc<InventoryUseCases> {
    fn from_ref(state: &AppState) -> Self { state.inventory_use_cases.clone() }
}

impl FromRef<AppState> for Arc<ProcurementUseCases> {
    fn from_ref(state: &AppState) -> Self { state.procurement_use_cases.clone() }
}

impl FromRef<AppState> for Arc<ReportUseCases> {
    fn from_ref(state: &AppState) -> Self { state.report_use_cases.clone() }
}

impl FromRef<AppState> for Arc<NotificationUseCases> {
    fn from_ref(state: &AppState) -> Self { state.notification_use_cases.clone() }
}
