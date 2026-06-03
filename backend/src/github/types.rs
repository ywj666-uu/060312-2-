use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitHubUser {
    pub login: String,
    pub avatar_url: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubRepo {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub open_issues_count: u64,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubPR {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub state: String,
    pub user: GitHubUser,
    pub comments: Option<u64>,
    pub created_at: String,
    pub merged_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubPRDetail {
    pub additions: u64,
    pub deletions: u64,
    pub changed_files: u64,
    pub comments: u64,
    pub merged_at: Option<String>,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubIssue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub state: String,
    pub user: GitHubUser,
    pub comments: u64,
    pub created_at: String,
    pub closed_at: Option<String>,
    pub pull_request: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubComment {
    pub id: u64,
    pub body: String,
    pub user: GitHubUser,
    pub created_at: String,
}
