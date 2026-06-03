use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub id: i64,
    pub user_id: i64,
    pub project_id: Option<i64>,
    pub lines_changed_score: i64,
    pub comments_score: i64,
    pub bonus_score: i64,
    pub total_score: i64,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct LeaderboardEntry {
    pub rank: u32,
    pub user_id: i64,
    pub github_username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub lines_changed_score: i64,
    pub comments_score: i64,
    pub bonus_score: i64,
    pub total_score: i64,
}

#[derive(Debug, Deserialize)]
pub struct BonusRequest {
    pub user_id: i64,
    pub project_id: Option<i64>,
    pub points: i64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BonusPoint {
    pub id: i64,
    pub user_id: i64,
    pub project_id: Option<i64>,
    pub points: i64,
    pub reason: String,
    pub granted_by: i64,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SetPasswordRequest {
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: i64,
    pub is_maintainer: bool,
}
