use actix_web::{web, HttpResponse, get, post};
use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use crate::db::queries;
use crate::errors::AppError;
use crate::github::sync::SyncService;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sync")
            .service(trigger_sync)
            .service(get_sync_status)
    );
}

#[post("")]
async fn trigger_sync(
    db: web::Data<Mutex<Connection>>,
    sync_svc: web::Data<Arc<SyncService>>,
) -> Result<HttpResponse, AppError> {
    let db_ref = db.into_inner();
    match sync_svc.sync_all(&db_ref).await {
        Ok(count) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "items_synced": count
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": e
        }))),
    }
}

#[get("/status")]
async fn get_sync_status(
    db: web::Data<Mutex<Connection>>,
) -> Result<HttpResponse, AppError> {
    let conn = db.lock().unwrap();
    let last_sync = queries::get_last_sync(&conn)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "last_sync": last_sync
    })))
}
