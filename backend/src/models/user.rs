use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub github_username: String,
    #[serde(skip_serializing)]
    pub github_pat: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_maintainer: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub github_username: String,
    pub github_pat: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub github_pat: Option<String>,
    pub display_name: Option<String>,
    pub is_maintainer: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub github_username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_maintainer: bool,
    pub total_score: i64,
}
