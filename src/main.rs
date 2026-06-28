use dotenvy::dotenv;
use pos::infra::{app::create_app, setup::init_app_state};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let app_state = init_app_state().await?;
    let bind_addr = app_state.config.bind_addr.clone();
    let app = create_app(app_state);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();
    info!("Backend listening at {}", &listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
