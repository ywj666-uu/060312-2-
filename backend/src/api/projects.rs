use actix_web::{web, HttpResponse, get, post, delete};
use std::sync::Mutex;
use rusqlite::Connection;
use crate::db::queries;
use crate::models::*;
use crate::errors::AppError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/projects")
            .service(add_project)
            .service(list_projects)
            .service(get_project)
            .service(delete_project)
    );
}

#[derive(serde::Serialize)]
struct ProjectWithHotness {
    #[serde(flatten)]
    project: Project,
    hotness: i64,
}

#[post("")]
async fn add_project(
    db: web::Data<Mutex<Connection>>,
    body: web::Json<AddProjectRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = db.lock().unwrap();
    let id = queries::insert_project(&conn, &body)?;
    let project = queries::get_project_by_id(&conn, id)?
        .ok_or(AppError::NotFound("Project not found".into()))?;
    Ok(HttpResponse::Created().json(project))
}

#[get("")]
async fn list_projects(
    db: web::Data<Mutex<Connection>>,
) -> Result<HttpResponse, AppError> {
    let conn = db.lock().unwrap();
    let projects_with_hotness = queries::list_projects_by_hotness(&conn)?;
    let results: Vec<ProjectWithHotness> = projects_with_hotness
        .into_iter()
        .map(|(project, hotness)| ProjectWithHotness { project, hotness })
        .collect();
    Ok(HttpResponse::Ok().json(results))
}

#[get("/{id}")]
async fn get_project(
    db: web::Data<Mutex<Connection>>,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let conn = db.lock().unwrap();
    let project = queries::get_project_by_id(&conn, id)?
        .ok_or(AppError::NotFound("Project not found".into()))?;
    let hotness = queries::get_project_hotness(&conn, id)?;
    Ok(HttpResponse::Ok().json(ProjectWithHotness { project, hotness }))
}

#[delete("/{id}")]
async fn delete_project(
    db: web::Data<Mutex<Connection>>,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let conn = db.lock().unwrap();
    queries::delete_project(&conn, id)?;
    Ok(HttpResponse::NoContent().finish())
}
