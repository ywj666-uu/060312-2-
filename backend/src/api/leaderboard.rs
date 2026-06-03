use actix_web::{web, HttpResponse, get};
use std::sync::Mutex;
use rusqlite::Connection;
use serde::Deserialize;
use crate::db::queries;
use crate::errors::AppError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/leaderboard")
            .service(get_leaderboard)
            .service(get_user_rank)
    );
}

#[derive(Deserialize)]
pub struct LeaderboardQuery {
    pub project_id: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[get("")]
async fn get_leaderboard(
    db: web::Data<Mutex<Connection>>,
    query: web::Query<LeaderboardQuery>,
) -> Result<HttpResponse, AppError> {
    let conn = db.lock().unwrap();
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    let entries = queries::get_leaderboard(&conn, query.project_id, limit, offset)?;
    Ok(HttpResponse::Ok().json(entries))
}

#[get("/user/{id}")]
async fn get_user_rank(
    db: web::Data<Mutex<Connection>>,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let conn = db.lock().unwrap();
    let score = queries::get_user_score(&conn, user_id, None)?;
    Ok(HttpResponse::Ok().json(score))
}
