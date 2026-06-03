use std::sync::Mutex;
use rusqlite::Connection;
use tracing::{info, error};
use super::client::GitHubClient;
use crate::db::queries;
use crate::models::*;

pub struct SyncService {
    pub github: GitHubClient,
}

impl SyncService {
    pub fn new(github_base_url: &str) -> Self {
        Self {
            github: GitHubClient::new(github_base_url),
        }
    }

    pub async fn sync_all(&self, db: &Mutex<Connection>) -> Result<u64, String> {
        let (users, projects, last_sync) = {
            let conn = db.lock().map_err(|e| e.to_string())?;
            let users = queries::list_users(&conn).map_err(|e| e.to_string())?;
            let projects = queries::list_projects(&conn).map_err(|e| e.to_string())?;
            let last_sync = queries::get_last_sync(&conn).unwrap_or(None);
            (users, projects, last_sync)
        };

        if users.is_empty() || projects.is_empty() {
            info!("No users or projects to sync");
            return Ok(0);
        }

        let since = last_sync.as_deref();
        info!("Incremental sync since: {:?}", since);

        let mut total_items: u64 = 0;

        for user in &users {
            for project in &projects {
                match self.sync_user_project(db, user, project, since).await {
                    Ok(count) => {
                        total_items += count;
                        info!("Synced {} items for user {} on {}/{}", count, user.github_username, project.owner, project.repo);
                    }
                    Err(e) => {
                        error!("Sync failed for user {} on {}/{}: {}", user.github_username, project.owner, project.repo, e);
                        let conn = db.lock().map_err(|e| e.to_string())?;
                        let _ = queries::insert_sync_log(&conn, Some(user.id), Some(project.id), "error", Some(&e), 0);
                    }
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }

        // Update project stats
        for project in &projects {
            if let Some(user) = users.first() {
                if let Ok(repo_info) = self.github.get_repo_info(&user.github_pat, &project.owner, &project.repo).await {
                    let conn = db.lock().map_err(|e| e.to_string())?;
                    let _ = queries::update_project_stats(
                        &conn, project.id,
                        repo_info.stargazers_count as i64,
                        repo_info.forks_count as i64,
                        repo_info.open_issues_count as i64,
                        repo_info.description.as_deref(),
                        repo_info.language.as_deref(),
                    );
                }
            }
        }

        // Recalculate scores
        self.recalculate_scores(db, &users, &projects)?;

        // Log success
        {
            let conn = db.lock().map_err(|e| e.to_string())?;
            let _ = queries::insert_sync_log(&conn, None, None, "success", None, total_items as i64);
        }

        Ok(total_items)
    }

    async fn sync_user_project(&self, db: &Mutex<Connection>, user: &User, project: &Project, since: Option<&str>) -> Result<u64, String> {
        let mut count: u64 = 0;

        // Sync PRs (incremental: sorted by updated desc, use since if available)
        let prs = self.github.get_pull_requests(&user.github_pat, &project.owner, &project.repo, since, 1)
            .await.map_err(|e| e.to_string())?;

        for pr in &prs {
            if pr.user.login != user.github_username {
                continue;
            }
            let detail = self.github.get_pr_detail(&user.github_pat, &project.owner, &project.repo, pr.number)
                .await.map_err(|e| e.to_string())?;

            let state = if detail.merged_at.is_some() { "merged".to_string() } else { detail.state.clone() };

            let pr_model = PullRequest {
                id: 0,
                github_id: pr.id as i64,
                project_id: project.id,
                user_id: user.id,
                title: pr.title.clone(),
                state,
                additions: detail.additions as i64,
                deletions: detail.deletions as i64,
                changed_files: detail.changed_files as i64,
                comments_count: detail.comments as i64,
                created_at: pr.created_at.clone(),
                merged_at: detail.merged_at.clone(),
            };

            {
                let conn = db.lock().map_err(|e| e.to_string())?;
                queries::upsert_pull_request(&conn, &pr_model).map_err(|e| e.to_string())?;
            }
            count += 1;

            // Sync review comments on this PR
            if let Ok(review_comments) = self.github.get_pr_review_comments(
                &user.github_pat, &project.owner, &project.repo, pr.number, since
            ).await {
                for comment in &review_comments {
                    if comment.user.login != user.github_username {
                        continue;
                    }
                    let comment_model = Comment {
                        id: 0,
                        github_id: comment.id as i64,
                        user_id: user.id,
                        project_id: project.id,
                        target_type: "pr".to_string(),
                        target_id: pr.id as i64,
                        body_length: comment.body.len() as i64,
                        created_at: comment.created_at.clone(),
                    };
                    let conn = db.lock().map_err(|e| e.to_string())?;
                    queries::upsert_comment(&conn, &comment_model).map_err(|e| e.to_string())?;
                    count += 1;
                }
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        // Sync issues (incremental, filter out PRs)
        let issues = self.github.get_issues(&user.github_pat, &project.owner, &project.repo, since, 1)
            .await.map_err(|e| e.to_string())?;

        for issue in &issues {
            if issue.pull_request.is_some() {
                continue;
            }
            if issue.user.login != user.github_username {
                continue;
            }

            let issue_model = Issue {
                id: 0,
                github_id: issue.id as i64,
                project_id: project.id,
                user_id: user.id,
                title: issue.title.clone(),
                state: issue.state.clone(),
                comments_count: issue.comments as i64,
                created_at: issue.created_at.clone(),
                closed_at: issue.closed_at.clone(),
            };

            {
                let conn = db.lock().map_err(|e| e.to_string())?;
                queries::upsert_issue(&conn, &issue_model).map_err(|e| e.to_string())?;
            }
            count += 1;

            // Sync comments on this issue (incremental)
            if issue.comments > 0 {
                if let Ok(comments) = self.github.get_issue_comments(
                    &user.github_pat, &project.owner, &project.repo, issue.number, since
                ).await {
                    for comment in &comments {
                        if comment.user.login != user.github_username {
                            continue;
                        }
                        let comment_model = Comment {
                            id: 0,
                            github_id: comment.id as i64,
                            user_id: user.id,
                            project_id: project.id,
                            target_type: "issue".to_string(),
                            target_id: issue.id as i64,
                            body_length: comment.body.len() as i64,
                            created_at: comment.created_at.clone(),
                        };
                        let conn = db.lock().map_err(|e| e.to_string())?;
                        queries::upsert_comment(&conn, &comment_model).map_err(|e| e.to_string())?;
                        count += 1;
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        // Update user avatar
        if let Ok(gh_user) = self.github.get_user_info(&user.github_pat, &user.github_username).await {
            let conn = db.lock().map_err(|e| e.to_string())?;
            let _ = queries::update_user_avatar(&conn, user.id, &gh_user.avatar_url);
        }

        Ok(count)
    }

    fn recalculate_scores(&self, db: &Mutex<Connection>, users: &[User], projects: &[Project]) -> Result<(), String> {
        let conn = db.lock().map_err(|e| e.to_string())?;
        for user in users {
            let lines = queries::compute_lines_score(&conn, user.id, None).unwrap_or(0);
            let comments = queries::compute_comments_score(&conn, user.id, None).unwrap_or(0);
            let bonus = queries::compute_bonus_total(&conn, user.id, None).unwrap_or(0);
            let _ = queries::upsert_score(&conn, user.id, None, lines, comments, bonus);

            for project in projects {
                let lines = queries::compute_lines_score(&conn, user.id, Some(project.id)).unwrap_or(0);
                let comments = queries::compute_comments_score(&conn, user.id, Some(project.id)).unwrap_or(0);
                let bonus = queries::compute_bonus_total(&conn, user.id, Some(project.id)).unwrap_or(0);
                let _ = queries::upsert_score(&conn, user.id, Some(project.id), lines, comments, bonus);
            }
        }
        Ok(())
    }
}
