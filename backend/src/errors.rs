use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Db(String),
    GitHub(String),
    NotFound(String),
    BadRequest(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Db(msg) => write!(f, "Database error: {}", msg),
            AppError::GitHub(msg) => write!(f, "GitHub API error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Db(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": self.to_string()
            })),
            AppError::GitHub(_) => HttpResponse::BadGateway().json(serde_json::json!({
                "error": self.to_string()
            })),
            AppError::NotFound(_) => HttpResponse::NotFound().json(serde_json::json!({
                "error": self.to_string()
            })),
            AppError::BadRequest(_) => HttpResponse::BadRequest().json(serde_json::json!({
                "error": self.to_string()
            })),
        }
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError::Db(e.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::GitHub(e.to_string())
    }
}
