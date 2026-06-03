use gloo_net::http::Request;
use crate::types::*;

const BASE: &str = "/api/v1";

fn auth_header(token: &str) -> String {
    format!("Bearer {}", token)
}

pub async fn list_users() -> Result<Vec<UserResponse>, String> {
    let resp = Request::get(&format!("{}/users", BASE))
        .send().await.map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn create_user(req: &CreateUserRequest) -> Result<UserResponse, String> {
    let resp = Request::post(&format!("{}/users", BASE))
        .json(req).map_err(|e| e.to_string())?
        .send().await.map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn list_projects() -> Result<Vec<Project>, String> {
    let resp = Request::get(&format!("{}/projects", BASE))
        .send().await.map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn add_project(req: &AddProjectRequest) -> Result<Project, String> {
    let resp = Request::post(&format!("{}/projects", BASE))
        .json(req).map_err(|e| e.to_string())?
        .send().await.map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn get_leaderboard(project_id: Option<i64>, limit: u32) -> Result<Vec<LeaderboardEntry>, String> {
    let mut url = format!("{}/leaderboard?limit={}", BASE, limit);
    if let Some(pid) = project_id {
        url.push_str(&format!("&project_id={}", pid));
    }
    let resp = Request::get(&url)
        .send().await.map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn get_user_score(user_id: i64) -> Result<Option<Score>, String> {
    let resp = Request::get(&format!("{}/leaderboard/user/{}", BASE, user_id))
        .send().await.map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn get_user_prs(user_id: i64) -> Result<Vec<PullRequest>, String> {
    let resp = Request::get(&format!("{}/contributions/prs?user_id={}", BASE, user_id))
        .send().await.map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn get_user_issues(user_id: i64) -> Result<Vec<Issue>, String> {
    let resp = Request::get(&format!("{}/contributions/issues?user_id={}", BASE, user_id))
        .send().await.map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn award_bonus(token: &str, req: &BonusRequest) -> Result<(), String> {
    let resp = Request::post(&format!("{}/bonus", BASE))
        .header("Authorization", &auth_header(token))
        .json(req).map_err(|e| e.to_string())?
        .send().await.map_err(|e| e.to_string())?;
    if resp.ok() {
        Ok(())
    } else {
        Err(format!("HTTP {}", resp.status()))
    }
}

pub async fn get_bonus_history(user_id: i64) -> Result<Vec<BonusPoint>, String> {
    let resp = Request::get(&format!("{}/bonus?user_id={}", BASE, user_id))
        .send().await.map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn login(req: &LoginRequest) -> Result<LoginResponse, String> {
    let resp = Request::post(&format!("{}/auth/login", BASE))
        .json(req).map_err(|e| e.to_string())?
        .send().await.map_err(|e| e.to_string())?;
    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err("Login failed".to_string())
    }
}

pub async fn logout(token: &str) -> Result<(), String> {
    Request::post(&format!("{}/auth/logout", BASE))
        .header("Authorization", &auth_header(token))
        .send().await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn trigger_sync() -> Result<SyncStatus, String> {
    let resp = Request::post(&format!("{}/sync", BASE))
        .send().await.map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn get_sync_status() -> Result<SyncStatus, String> {
    let resp = Request::get(&format!("{}/sync/status", BASE))
        .send().await.map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}
