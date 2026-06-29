use std::sync::Arc;

use tokio::time::{Duration, interval};
use tracing::{error, info};

use crate::application::use_cases::auth::AuthUseCases;

pub fn spawn_sync_worker(auth_use_cases: Arc<AuthUseCases>) {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(60));
        loop {
            ticker.tick().await;

            info!("Sync worker tick: starting sync");

            // Get the last sync timestamp — defaults to Unix epoch if never synced
            let since = match auth_use_cases.get_last_sync_timestamp().await {
                Ok(Some(ts)) => ts,
                Ok(None) => {
                    // Never synced — use a very old date to pull everything
                    chrono::DateTime::from_timestamp(0, 0).unwrap()
                }
                Err(e) => {
                    error!("Failed to get last sync timestamp: {}", e);
                    continue;
                }
            };

            if let Err(e) = auth_use_cases.sync_users(since).await {
                error!("Sync worker error: {}", e);
            }

            info!("Sync worker tick: completed");
        }
    });
}
