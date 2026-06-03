use actix_web::{web, HttpResponse, get, post, put, delete};
use std::sync::Mutex;
use rusqlite::Connection;
use crate::db::queries;
use crate::models::*;
use crate::errors::AppError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(create_user)
            .service(list_users)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
    );
}

#[post("")]
async fn create_user(
    db: web::Data<Mutex<Connection>>,
    body: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = db.lock().unwrap();
    let id = queries::insert_user(&conn, &body)?;
    let user = queries::get_user_by_id(&conn, id)?
        .ok_or(AppError::NotFound("User not found".into()))?;
    Ok(HttpResponse::Created().json(UserResponse {
        id: user.id,
        github_username: user.github_username,
        display_name: user.display_name,
        avatar_url: user.avatar_url,
        is_maintainer: user.is_maintainer,
        total_score: 0,
    }))
}

#[get("")]
async fn list_users(
    db: web::Data<Mutex<Connection>>,
) -> Result<HttpResponse, AppError> {
    let conn = db.lock().unwrap();
    let users = queries::list_users(&conn)?;
    let responses: Vec<UserResponse> = users.into_iter().map(|u| {
        let score = queries::get_user_score(&conn, u.id, None)
            .unwrap_or(None)
            .map(|s| s.total_score)
            .unwrap_or(0);
        UserResponse {
            id: u.id,
            github_username: u.github_username,
            display_name: u.display_name,
            avatar_url: u.avatar_url,
            is_maintainer: u.is_maintainer,
            total_score: score,
        }
    }).collect();
    Ok(HttpResponse::Ok().json(responses))
}

#[get("/{id}")]
async fn get_user(
    db: web::Data<Mutex<Connection>>,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let conn = db.lock().unwrap();
    let user = queries::get_user_by_id(&conn, id)?
        .ok_or(AppError::NotFound("User not found".into()))?;
    let score = queries::get_user_score(&conn, id, None)
        .unwrap_or(None)
        .map(|s| s.total_score)
        .unwrap_or(0);
    Ok(HttpResponse::Ok().json(UserResponse {
        id: user.id,
        github_username: user.github_username,
        display_name: user.display_name,
        avatar_url: user.avatar_url,
        is_maintainer: user.is_maintainer,
        total_score: score,
    }))
}

#[put("/{id}")]
async fn update_user(
    db: web::Data<Mutex<Connection>>,
    path: web::Path<i64>,
    body: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let conn = db.lock().unwrap();
    queries::update_user(&conn, id, &body)?;
    let user = queries::get_user_by_id(&conn, id)?
        .ok_or(AppError::NotFound("User not found".into()))?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/{id}")]
async fn delete_user(
    db: web::Data<Mutex<Connection>>,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let conn = db.lock().unwrap();
    queries::delete_user(&conn, id)?;
    Ok(HttpResponse::NoContent().finish())
}
