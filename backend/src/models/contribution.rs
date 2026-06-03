use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    pub github_id: i64,
    pub user_id: i64,
    pub project_id: i64,
    pub target_type: String,
    pub target_id: i64,
    pub body_length: i64,
    pub created_at: String,
}
