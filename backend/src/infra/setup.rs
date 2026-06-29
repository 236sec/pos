use std::{fs::File, sync::Arc};

use uuid::Uuid;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    adapters::{http::app_state::AppState, persistence::PostgresPersistence},
    application::use_cases::{
        auth::AuthUseCases, inventory::InventoryUseCases, menu::MenuUseCases,
        notification::NotificationUseCases, pos::PosUseCases, procurement::ProcurementUseCases,
        report::ReportUseCases,
    },
    infra::{config::AppConfig, db::init_db, sync::spawn_sync_worker},
};

pub async fn init_app_state() -> anyhow::Result<AppState> {
    let config = AppConfig::from_env();
    let pool = init_db(&config.database_url).await?;
    let persistence = Arc::new(PostgresPersistence::new(pool));

    let branch_id = Uuid::parse_str(&config.branch_id)
        .expect("BRANCH_ID must be a valid UUID");

    let auth_use_cases = Arc::new(AuthUseCases::new(
        persistence.clone(),
        config.jwt_secret.clone(),
        branch_id,
        config.head_office_url.clone(),
    ));
    let pos_use_cases = Arc::new(PosUseCases::new(persistence.clone()));
    let menu_use_cases = Arc::new(MenuUseCases::new(persistence.clone()));
    let inventory_use_cases = Arc::new(InventoryUseCases::new(persistence.clone()));
    let procurement_use_cases = Arc::new(ProcurementUseCases::new(persistence.clone()));
    let report_use_cases = Arc::new(ReportUseCases::new(persistence.clone()));
    let notification_use_cases = Arc::new(NotificationUseCases::new(persistence.clone()));

    // Spawn the background sync worker for auth credential caching
    spawn_sync_worker(auth_use_cases.clone());

    Ok(AppState {
        config: Arc::new(config),
        auth_use_cases,
        pos_use_cases,
        menu_use_cases,
        inventory_use_cases,
        procurement_use_cases,
        report_use_cases,
        notification_use_cases,
    })
}

pub fn init_tracing() {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| "pos=debug,tower_http=debug".into());

    let console_layer = fmt::layer().with_target(false).with_level(true).pretty();

    let file = File::create("app.log").expect("cannot create log file");
    let json_layer = fmt::layer()
        .json()
        .with_writer(file)
        .with_current_span(true)
        .with_span_list(true);

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(json_layer)
        .try_init()
        .ok();
}
