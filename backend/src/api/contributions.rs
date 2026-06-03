use actix_web::{web, HttpResponse, get};
use std::sync::Mutex;
use rusqlite::Connection;
use serde::Deserialize;
use crate::db::queries;
use crate::errors::AppError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/contributions")
            .service(get_prs)
            .service(get_issues)
            .service(get_comments)
    );
}

#[derive(Deserialize)]
pub struct ContribQuery {
    pub user_id: i64,
    pub project_id: Option<i64>,
}

#[get("/prs")]
async fn get_prs(
    db: web::Data<Mutex<Connection>>,
    query: web::Query<ContribQuery>,
) -> Result<HttpResponse, AppError> {
    let conn = db.lock().unwrap();
    let prs = queries::get_prs_by_user(&conn, query.user_id, query.project_id)?;
    Ok(HttpResponse::Ok().json(prs))
}

#[get("/issues")]
async fn get_issues(
    db: web::Data<Mutex<Connection>>,
    query: web::Query<ContribQuery>,
) -> Result<HttpResponse, AppError> {
    let conn = db.lock().unwrap();
    let issues = queries::get_issues_by_user(&conn, query.user_id, query.project_id)?;
    Ok(HttpResponse::Ok().json(issues))
}

#[get("/comments")]
async fn get_comments(
    db: web::Data<Mutex<Connection>>,
    query: web::Query<ContribQuery>,
) -> Result<HttpResponse, AppError> {
    let conn = db.lock().unwrap();
    let comments = queries::get_comments_by_user(&conn, query.user_id, query.project_id)?;
    Ok(HttpResponse::Ok().json(comments))
}
