use actix_web::{web, HttpResponse, HttpRequest, get, post, delete};
use std::sync::Mutex;
use rusqlite::Connection;
use serde::Deserialize;
use crate::db::queries;
use crate::models::*;
use crate::errors::AppError;
use crate::scoring::calculator::recalculate_all_scores;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bonus")
            .service(award_bonus)
            .service(get_bonus)
            .service(delete_bonus)
    );
    cfg.service(
        web::scope("/auth")
            .service(login)
            .service(set_password)
            .service(logout)
    );
}

fn extract_session_user(req: &HttpRequest, db: &Mutex<Connection>) -> Result<User, AppError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::BadRequest("Missing Authorization header".into()))?;

    let conn = db.lock().unwrap();
    queries::validate_session(&conn, token)?
        .ok_or_else(|| AppError::BadRequest("Invalid or expired session".into()))
}

fn require_maintainer(req: &HttpRequest, db: &Mutex<Connection>) -> Result<User, AppError> {
    let user = extract_session_user(req, db)?;
    if !user.is_maintainer {
        return Err(AppError::BadRequest("Only maintainers can perform this action".into()));
    }
    Ok(user)
}

#[derive(Deserialize)]
pub struct BonusQuery {
    pub user_id: i64,
}

#[post("")]
async fn award_bonus(
    req: HttpRequest,
    db: web::Data<Mutex<Connection>>,
    body: web::Json<BonusRequest>,
) -> Result<HttpResponse, AppError> {
    let maintainer = require_maintainer(&req, &db)?;
    {
        let conn = db.lock().unwrap();
        queries::insert_bonus(&conn, body.user_id, body.project_id, body.points, &body.reason, maintainer.id)?;
    }
    let _ = recalculate_all_scores(&db);
    Ok(HttpResponse::Created().json(serde_json::json!({"status": "ok"})))
}

#[get("")]
async fn get_bonus(
    db: web::Data<Mutex<Connection>>,
    query: web::Query<BonusQuery>,
) -> Result<HttpResponse, AppError> {
    let conn = db.lock().unwrap();
    let bonuses = queries::get_bonus_by_user(&conn, query.user_id)?;
    Ok(HttpResponse::Ok().json(bonuses))
}

#[delete("/{id}")]
async fn delete_bonus(
    req: HttpRequest,
    db: web::Data<Mutex<Connection>>,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let _maintainer = require_maintainer(&req, &db)?;
    {
        let conn = db.lock().unwrap();
        queries::delete_bonus(&conn, path.into_inner())?;
    }
    let _ = recalculate_all_scores(&db);
    Ok(HttpResponse::NoContent().finish())
}

// --- Auth endpoints ---

#[post("/login")]
async fn login(
    db: web::Data<Mutex<Connection>>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = db.lock().unwrap();
    let user = queries::get_user_by_username(&conn, &body.username)?
        .ok_or_else(|| AppError::BadRequest("User not found".into()))?;

    let stored_hash = queries::get_user_password_hash(&conn, user.id)?
        .ok_or_else(|| AppError::BadRequest("Password not set. Use set-password first.".into()))?;

    if !verify_password(&body.password, &stored_hash) {
        return Err(AppError::BadRequest("Invalid password".into()));
    }

    let token = generate_token();
    queries::create_session(&conn, user.id, &token)?;

    Ok(HttpResponse::Ok().json(LoginResponse {
        token,
        user_id: user.id,
        is_maintainer: user.is_maintainer,
    }))
}

#[post("/set-password")]
async fn set_password(
    req: HttpRequest,
    db: web::Data<Mutex<Connection>>,
    body: web::Json<SetPasswordRequest>,
) -> Result<HttpResponse, AppError> {
    let user = extract_session_user(&req, &db)?;
    let hash = hash_password(&body.password);
    let conn = db.lock().unwrap();
    queries::set_user_password(&conn, user.id, &hash)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"status": "ok"})))
}

#[post("/logout")]
async fn logout(
    req: HttpRequest,
    db: web::Data<Mutex<Connection>>,
) -> Result<HttpResponse, AppError> {
    if let Some(token) = req.headers().get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
    {
        let conn = db.lock().unwrap();
        let _ = queries::delete_session(&conn, token);
    }
    Ok(HttpResponse::Ok().json(serde_json::json!({"status": "ok"})))
}

fn generate_token() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("{:x}{:x}", timestamp, rand_simple())
}

fn rand_simple() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    t.as_nanos() as u64 ^ (t.subsec_nanos() as u64).wrapping_mul(6364136223846793005)
}

fn hash_password(password: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    "salt_contributor_platform".hash(&mut hasher);
    password.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn verify_password(password: &str, hash: &str) -> bool {
    hash_password(password) == hash
}
