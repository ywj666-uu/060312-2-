mod config;
mod db;
mod models;
mod api;
mod github;
mod scoring;
mod errors;

use actix_web::{web, App, HttpServer};
use actix_files::Files;
use std::sync::{Arc, Mutex};
use tracing::info;

use crate::config::AppConfig;
use crate::github::sync::SyncService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let config = AppConfig::load();
    info!("Starting server on {}:{}", config.server_host, config.server_port);
    info!("Database: {}", config.database_path);
    info!("Sync interval: {}s", config.sync_interval_seconds);

    // SQLite is the persistent cache — on restart we read existing data
    let conn = db::init_db(&config.database_path);
    let db_data = web::Data::new(Mutex::new(conn));

    let sync_service = Arc::new(SyncService::new(&config.github_api_base_url));
    let sync_data = web::Data::new(sync_service.clone());

    // Spawn background sync: runs IMMEDIATELY on startup, then every sync_interval
    let bg_db = db_data.clone();
    let bg_sync = sync_service.clone();
    let sync_interval = config.sync_interval_seconds;
    tokio::spawn(async move {
        // Initial sync on startup (incremental — uses last_synced_at from cache)
        info!("Running initial sync on startup (incremental from cache)...");
        match bg_sync.sync_all(&bg_db).await {
            Ok(count) => info!("Initial sync complete: {} new items", count),
            Err(e) => tracing::error!("Initial sync failed: {}", e),
        }

        // Periodic sync every interval
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(sync_interval)).await;
            info!("Starting periodic sync...");
            match bg_sync.sync_all(&bg_db).await {
                Ok(count) => info!("Periodic sync complete: {} items", count),
                Err(e) => tracing::error!("Periodic sync failed: {}", e),
            }
        }
    });

    let frontend_path = config.frontend_dist_path.clone();
    let host = config.server_host.clone();
    let port = config.server_port;

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .app_data(sync_data.clone())
            .configure(api::configure)
            .service(Files::new("/", &frontend_path).index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await
}
