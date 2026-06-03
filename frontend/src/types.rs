use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i64,
    pub github_username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_maintainer: bool,
    pub total_score: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub owner: String,
    pub repo: String,
    pub description: Option<String>,
    pub stars: i64,
    pub forks: i64,
    pub open_issues: i64,
    pub language: Option<String>,
    pub last_synced_at: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub hotness: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BonusPoint {
    pub id: i64,
    pub user_id: i64,
    pub project_id: Option<i64>,
    pub points: i64,
    pub reason: String,
    pub granted_by: String,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: i64,
    pub github_id: i64,
    pub project_id: i64,
    pub user_id: i64,
    pub title: String,
    pub state: String,
    pub additions: i64,
    pub deletions: i64,
    pub changed_files: i64,
    pub comments_count: i64,
    pub created_at: String,
    pub merged_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Issue {
    pub id: i64,
    pub github_id: i64,
    pub project_id: i64,
    pub user_id: i64,
    pub title: String,
    pub state: String,
    pub comments_count: i64,
    pub created_at: String,
    pub closed_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncStatus {
    pub last_sync: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub github_username: String,
    pub github_pat: String,
    pub display_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddProjectRequest {
    pub owner: String,
    pub repo: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BonusRequest {
    pub user_id: i64,
    pub project_id: Option<i64>,
    pub points: i64,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: i64,
    pub is_maintainer: bool,
}
