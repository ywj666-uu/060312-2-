use reqwest::Client;
use super::types::*;
use crate::errors::AppError;

pub struct GitHubClient {
    http: Client,
    base_url: String,
}

impl GitHubClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    fn auth_headers(&self, pat: &str) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", pat).parse().unwrap());
        headers.insert("User-Agent", "contributor-platform".parse().unwrap());
        headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
        headers
    }

    pub async fn get_user_info(&self, pat: &str, username: &str) -> Result<GitHubUser, AppError> {
        let url = format!("{}/users/{}", self.base_url, username);
        let resp = self.http.get(&url)
            .headers(self.auth_headers(pat))
            .send().await?
            .error_for_status()
            .map_err(|e| AppError::GitHub(e.to_string()))?;
        Ok(resp.json().await?)
    }

    pub async fn get_repo_info(&self, pat: &str, owner: &str, repo: &str) -> Result<GitHubRepo, AppError> {
        let url = format!("{}/repos/{}/{}", self.base_url, owner, repo);
        let resp = self.http.get(&url)
            .headers(self.auth_headers(pat))
            .send().await?
            .error_for_status()
            .map_err(|e| AppError::GitHub(e.to_string()))?;
        Ok(resp.json().await?)
    }

    pub async fn get_pull_requests(&self, pat: &str, owner: &str, repo: &str, since: Option<&str>, page: u32) -> Result<Vec<GitHubPR>, AppError> {
        let mut url = format!(
            "{}/repos/{}/{}/pulls?state=all&sort=updated&direction=desc&per_page=100&page={}",
            self.base_url, owner, repo, page
        );
        if let Some(s) = since {
            url.push_str(&format!("&since={}", s));
        }
        let resp = self.http.get(&url)
            .headers(self.auth_headers(pat))
            .send().await?
            .error_for_status()
            .map_err(|e| AppError::GitHub(e.to_string()))?;
        Ok(resp.json().await?)
    }

    pub async fn get_pr_detail(&self, pat: &str, owner: &str, repo: &str, number: u64) -> Result<GitHubPRDetail, AppError> {
        let url = format!("{}/repos/{}/{}/pulls/{}", self.base_url, owner, repo, number);
        let resp = self.http.get(&url)
            .headers(self.auth_headers(pat))
            .send().await?
            .error_for_status()
            .map_err(|e| AppError::GitHub(e.to_string()))?;
        Ok(resp.json().await?)
    }

    pub async fn get_issues(&self, pat: &str, owner: &str, repo: &str, since: Option<&str>, page: u32) -> Result<Vec<GitHubIssue>, AppError> {
        let mut url = format!(
            "{}/repos/{}/{}/issues?state=all&sort=updated&direction=desc&per_page=100&page={}",
            self.base_url, owner, repo, page
        );
        if let Some(s) = since {
            url.push_str(&format!("&since={}", s));
        }
        let resp = self.http.get(&url)
            .headers(self.auth_headers(pat))
            .send().await?
            .error_for_status()
            .map_err(|e| AppError::GitHub(e.to_string()))?;
        Ok(resp.json().await?)
    }

    pub async fn get_issue_comments(&self, pat: &str, owner: &str, repo: &str, issue_number: u64, since: Option<&str>) -> Result<Vec<GitHubComment>, AppError> {
        let mut url = format!(
            "{}/repos/{}/{}/issues/{}/comments?per_page=100",
            self.base_url, owner, repo, issue_number
        );
        if let Some(s) = since {
            url.push_str(&format!("&since={}", s));
        }
        let resp = self.http.get(&url)
            .headers(self.auth_headers(pat))
            .send().await?
            .error_for_status()
            .map_err(|e| AppError::GitHub(e.to_string()))?;
        Ok(resp.json().await?)
    }

    pub async fn get_pr_review_comments(&self, pat: &str, owner: &str, repo: &str, pr_number: u64, since: Option<&str>) -> Result<Vec<GitHubComment>, AppError> {
        let mut url = format!(
            "{}/repos/{}/{}/pulls/{}/comments?per_page=100",
            self.base_url, owner, repo, pr_number
        );
        if let Some(s) = since {
            url.push_str(&format!("&since={}", s));
        }
        let resp = self.http.get(&url)
            .headers(self.auth_headers(pat))
            .send().await?
            .error_for_status()
            .map_err(|e| AppError::GitHub(e.to_string()))?;
        Ok(resp.json().await?)
    }
}
