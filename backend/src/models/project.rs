use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

#[derive(Debug, Deserialize)]
pub struct AddProjectRequest {
    pub owner: String,
    pub repo: String,
}
