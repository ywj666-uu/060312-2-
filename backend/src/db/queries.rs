use rusqlite::{params, Connection, OptionalExtension};
use crate::models::*;

// --- User queries ---

pub fn insert_user(conn: &Connection, req: &CreateUserRequest) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO users (github_username, github_pat, display_name) VALUES (?1, ?2, ?3)",
        params![req.github_username, req.github_pat, req.display_name],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_user_by_id(conn: &Connection, id: i64) -> rusqlite::Result<Option<User>> {
    conn.query_row(
        "SELECT id, github_username, github_pat, display_name, avatar_url, is_maintainer, created_at, updated_at FROM users WHERE id = ?1",
        params![id],
        |row| Ok(User {
            id: row.get(0)?,
            github_username: row.get(1)?,
            github_pat: row.get(2)?,
            display_name: row.get(3)?,
            avatar_url: row.get(4)?,
            is_maintainer: row.get::<_, i32>(5)? != 0,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        }),
    ).optional()
}

pub fn list_users(conn: &Connection) -> rusqlite::Result<Vec<User>> {
    let mut stmt = conn.prepare(
        "SELECT id, github_username, github_pat, display_name, avatar_url, is_maintainer, created_at, updated_at FROM users ORDER BY id"
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            github_username: row.get(1)?,
            github_pat: row.get(2)?,
            display_name: row.get(3)?,
            avatar_url: row.get(4)?,
            is_maintainer: row.get::<_, i32>(5)? != 0,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    rows.collect()
}

pub fn update_user(conn: &Connection, id: i64, req: &UpdateUserRequest) -> rusqlite::Result<()> {
    if let Some(ref pat) = req.github_pat {
        conn.execute("UPDATE users SET github_pat = ?1, updated_at = datetime('now') WHERE id = ?2", params![pat, id])?;
    }
    if let Some(ref name) = req.display_name {
        conn.execute("UPDATE users SET display_name = ?1, updated_at = datetime('now') WHERE id = ?2", params![name, id])?;
    }
    if let Some(is_m) = req.is_maintainer {
        conn.execute("UPDATE users SET is_maintainer = ?1, updated_at = datetime('now') WHERE id = ?2", params![is_m as i32, id])?;
    }
    Ok(())
}

pub fn update_user_avatar(conn: &Connection, id: i64, avatar_url: &str) -> rusqlite::Result<()> {
    conn.execute("UPDATE users SET avatar_url = ?1, updated_at = datetime('now') WHERE id = ?2", params![avatar_url, id])?;
    Ok(())
}

pub fn delete_user(conn: &Connection, id: i64) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM users WHERE id = ?1", params![id])?;
    Ok(())
}

// --- Project queries ---

pub fn insert_project(conn: &Connection, req: &AddProjectRequest) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO projects (owner, repo) VALUES (?1, ?2)",
        params![req.owner, req.repo],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_project_by_id(conn: &Connection, id: i64) -> rusqlite::Result<Option<Project>> {
    conn.query_row(
        "SELECT id, owner, repo, description, stars, forks, open_issues, language, last_synced_at, created_at FROM projects WHERE id = ?1",
        params![id],
        |row| Ok(Project {
            id: row.get(0)?,
            owner: row.get(1)?,
            repo: row.get(2)?,
            description: row.get(3)?,
            stars: row.get(4)?,
            forks: row.get(5)?,
            open_issues: row.get(6)?,
            language: row.get(7)?,
            last_synced_at: row.get(8)?,
            created_at: row.get(9)?,
        }),
    ).optional()
}

pub fn list_projects(conn: &Connection) -> rusqlite::Result<Vec<Project>> {
    let mut stmt = conn.prepare(
        "SELECT id, owner, repo, description, stars, forks, open_issues, language, last_synced_at, created_at FROM projects ORDER BY stars DESC"
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            owner: row.get(1)?,
            repo: row.get(2)?,
            description: row.get(3)?,
            stars: row.get(4)?,
            forks: row.get(5)?,
            open_issues: row.get(6)?,
            language: row.get(7)?,
            last_synced_at: row.get(8)?,
            created_at: row.get(9)?,
        })
    })?;
    rows.collect()
}

pub fn update_project_stats(conn: &Connection, id: i64, stars: i64, forks: i64, open_issues: i64, description: Option<&str>, language: Option<&str>) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE projects SET stars = ?1, forks = ?2, open_issues = ?3, description = ?4, language = ?5, last_synced_at = datetime('now') WHERE id = ?6",
        params![stars, forks, open_issues, description, language, id],
    )?;
    Ok(())
}

pub fn delete_project(conn: &Connection, id: i64) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM projects WHERE id = ?1", params![id])?;
    Ok(())
}

// --- Pull Request queries ---

pub fn upsert_pull_request(conn: &Connection, pr: &PullRequest) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO pull_requests (github_id, project_id, user_id, title, state, additions, deletions, changed_files, comments_count, created_at, merged_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
         ON CONFLICT(github_id, project_id) DO UPDATE SET
            state = excluded.state,
            additions = excluded.additions,
            deletions = excluded.deletions,
            changed_files = excluded.changed_files,
            comments_count = excluded.comments_count,
            merged_at = excluded.merged_at",
        params![pr.github_id, pr.project_id, pr.user_id, pr.title, pr.state, pr.additions, pr.deletions, pr.changed_files, pr.comments_count, pr.created_at, pr.merged_at],
    )?;
    Ok(())
}

pub fn get_prs_by_user(conn: &Connection, user_id: i64, project_id: Option<i64>) -> rusqlite::Result<Vec<PullRequest>> {
    let sql = if project_id.is_some() {
        "SELECT id, github_id, project_id, user_id, title, state, additions, deletions, changed_files, comments_count, created_at, merged_at FROM pull_requests WHERE user_id = ?1 AND project_id = ?2 ORDER BY created_at DESC"
    } else {
        "SELECT id, github_id, project_id, user_id, title, state, additions, deletions, changed_files, comments_count, created_at, merged_at FROM pull_requests WHERE user_id = ?1 ORDER BY created_at DESC"
    };
    let mut stmt = conn.prepare(sql)?;
    let rows = if let Some(pid) = project_id {
        stmt.query_map(params![user_id, pid], map_pr)?
    } else {
        stmt.query_map(params![user_id], map_pr)?
    };
    rows.collect()
}

fn map_pr(row: &rusqlite::Row) -> rusqlite::Result<PullRequest> {
    Ok(PullRequest {
        id: row.get(0)?,
        github_id: row.get(1)?,
        project_id: row.get(2)?,
        user_id: row.get(3)?,
        title: row.get(4)?,
        state: row.get(5)?,
        additions: row.get(6)?,
        deletions: row.get(7)?,
        changed_files: row.get(8)?,
        comments_count: row.get(9)?,
        created_at: row.get(10)?,
        merged_at: row.get(11)?,
    })
}

// --- Issue queries ---

pub fn upsert_issue(conn: &Connection, issue: &Issue) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO issues (github_id, project_id, user_id, title, state, comments_count, created_at, closed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
         ON CONFLICT(github_id, project_id) DO UPDATE SET
            state = excluded.state,
            comments_count = excluded.comments_count,
            closed_at = excluded.closed_at",
        params![issue.github_id, issue.project_id, issue.user_id, issue.title, issue.state, issue.comments_count, issue.created_at, issue.closed_at],
    )?;
    Ok(())
}

pub fn get_issues_by_user(conn: &Connection, user_id: i64, project_id: Option<i64>) -> rusqlite::Result<Vec<Issue>> {
    let sql = if project_id.is_some() {
        "SELECT id, github_id, project_id, user_id, title, state, comments_count, created_at, closed_at FROM issues WHERE user_id = ?1 AND project_id = ?2 ORDER BY created_at DESC"
    } else {
        "SELECT id, github_id, project_id, user_id, title, state, comments_count, created_at, closed_at FROM issues WHERE user_id = ?1 ORDER BY created_at DESC"
    };
    let mut stmt = conn.prepare(sql)?;
    let rows = if let Some(pid) = project_id {
        stmt.query_map(params![user_id, pid], map_issue)?
    } else {
        stmt.query_map(params![user_id], map_issue)?
    };
    rows.collect()
}

fn map_issue(row: &rusqlite::Row) -> rusqlite::Result<Issue> {
    Ok(Issue {
        id: row.get(0)?,
        github_id: row.get(1)?,
        project_id: row.get(2)?,
        user_id: row.get(3)?,
        title: row.get(4)?,
        state: row.get(5)?,
        comments_count: row.get(6)?,
        created_at: row.get(7)?,
        closed_at: row.get(8)?,
    })
}

// --- Comment queries ---

pub fn upsert_comment(conn: &Connection, comment: &Comment) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO comments (github_id, user_id, project_id, target_type, target_id, body_length, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(github_id) DO UPDATE SET body_length = excluded.body_length",
        params![comment.github_id, comment.user_id, comment.project_id, comment.target_type, comment.target_id, comment.body_length, comment.created_at],
    )?;
    Ok(())
}

pub fn get_comments_by_user(conn: &Connection, user_id: i64, project_id: Option<i64>) -> rusqlite::Result<Vec<Comment>> {
    let sql = if project_id.is_some() {
        "SELECT id, github_id, user_id, project_id, target_type, target_id, body_length, created_at FROM comments WHERE user_id = ?1 AND project_id = ?2 ORDER BY created_at DESC"
    } else {
        "SELECT id, github_id, user_id, project_id, target_type, target_id, body_length, created_at FROM comments WHERE user_id = ?1 ORDER BY created_at DESC"
    };
    let mut stmt = conn.prepare(sql)?;
    let rows = if let Some(pid) = project_id {
        stmt.query_map(params![user_id, pid], map_comment)?
    } else {
        stmt.query_map(params![user_id], map_comment)?
    };
    rows.collect()
}

fn map_comment(row: &rusqlite::Row) -> rusqlite::Result<Comment> {
    Ok(Comment {
        id: row.get(0)?,
        github_id: row.get(1)?,
        user_id: row.get(2)?,
        project_id: row.get(3)?,
        target_type: row.get(4)?,
        target_id: row.get(5)?,
        body_length: row.get(6)?,
        created_at: row.get(7)?,
    })
}

// --- Bonus points queries ---

pub fn insert_bonus(conn: &Connection, user_id: i64, project_id: Option<i64>, points: i64, reason: &str, granted_by: i64) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO bonus_points (user_id, project_id, points, reason, granted_by) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![user_id, project_id, points, reason, granted_by],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_bonus_by_user(conn: &Connection, user_id: i64) -> rusqlite::Result<Vec<BonusPoint>> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, project_id, points, reason, granted_by, created_at FROM bonus_points WHERE user_id = ?1 ORDER BY created_at DESC"
    )?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(BonusPoint {
            id: row.get(0)?,
            user_id: row.get(1)?,
            project_id: row.get(2)?,
            points: row.get(3)?,
            reason: row.get(4)?,
            granted_by: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    rows.collect()
}

pub fn delete_bonus(conn: &Connection, id: i64) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM bonus_points WHERE id = ?1", params![id])?;
    Ok(())
}

// --- Score queries ---

pub fn upsert_score(conn: &Connection, user_id: i64, project_id: Option<i64>, lines: i64, comments: i64, bonus: i64) -> rusqlite::Result<()> {
    let total = lines + comments + bonus;
    conn.execute(
        "INSERT INTO scores (user_id, project_id, lines_changed_score, comments_score, bonus_score, total_score, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'))
         ON CONFLICT(user_id, project_id) DO UPDATE SET
            lines_changed_score = excluded.lines_changed_score,
            comments_score = excluded.comments_score,
            bonus_score = excluded.bonus_score,
            total_score = excluded.total_score,
            updated_at = datetime('now')",
        params![user_id, project_id, lines, comments, bonus, total],
    )?;
    Ok(())
}

pub fn get_leaderboard(conn: &Connection, project_id: Option<i64>, limit: i64, offset: i64) -> rusqlite::Result<Vec<LeaderboardEntry>> {
    let mut entries = Vec::new();
    let mut rank: u32 = offset as u32;

    let map_row = |row: &rusqlite::Row| -> rusqlite::Result<LeaderboardEntry> {
        Ok(LeaderboardEntry {
            rank: 0,
            user_id: row.get(0)?,
            github_username: row.get(1)?,
            display_name: row.get(2)?,
            avatar_url: row.get(3)?,
            lines_changed_score: row.get(4)?,
            comments_score: row.get(5)?,
            bonus_score: row.get(6)?,
            total_score: row.get(7)?,
        })
    };

    if let Some(pid) = project_id {
        let mut stmt = conn.prepare(
            "SELECT s.user_id, u.github_username, u.display_name, u.avatar_url, s.lines_changed_score, s.comments_score, s.bonus_score, s.total_score
             FROM scores s JOIN users u ON u.id = s.user_id
             WHERE s.project_id = ?1 ORDER BY s.total_score DESC LIMIT ?2 OFFSET ?3"
        )?;
        let rows = stmt.query_map(params![pid, limit, offset], map_row)?;
        for r in rows {
            let mut entry = r?;
            rank += 1;
            entry.rank = rank;
            entries.push(entry);
        }
    } else {
        let mut stmt = conn.prepare(
            "SELECT s.user_id, u.github_username, u.display_name, u.avatar_url, s.lines_changed_score, s.comments_score, s.bonus_score, s.total_score
             FROM scores s JOIN users u ON u.id = s.user_id
             WHERE s.project_id IS NULL ORDER BY s.total_score DESC LIMIT ?1 OFFSET ?2"
        )?;
        let rows = stmt.query_map(params![limit, offset], map_row)?;
        for r in rows {
            let mut entry = r?;
            rank += 1;
            entry.rank = rank;
            entries.push(entry);
        }
    }
    Ok(entries)
}

pub fn get_user_score(conn: &Connection, user_id: i64, project_id: Option<i64>) -> rusqlite::Result<Option<Score>> {
    let sql = if project_id.is_some() {
        "SELECT id, user_id, project_id, lines_changed_score, comments_score, bonus_score, total_score, updated_at FROM scores WHERE user_id = ?1 AND project_id = ?2"
    } else {
        "SELECT id, user_id, project_id, lines_changed_score, comments_score, bonus_score, total_score, updated_at FROM scores WHERE user_id = ?1 AND project_id IS NULL"
    };

    if let Some(pid) = project_id {
        conn.query_row(sql, params![user_id, pid], |row| {
            Ok(Score {
                id: row.get(0)?,
                user_id: row.get(1)?,
                project_id: row.get(2)?,
                lines_changed_score: row.get(3)?,
                comments_score: row.get(4)?,
                bonus_score: row.get(5)?,
                total_score: row.get(6)?,
                updated_at: row.get(7)?,
            })
        }).optional()
    } else {
        conn.query_row(sql, params![user_id], |row| {
            Ok(Score {
                id: row.get(0)?,
                user_id: row.get(1)?,
                project_id: row.get(2)?,
                lines_changed_score: row.get(3)?,
                comments_score: row.get(4)?,
                bonus_score: row.get(5)?,
                total_score: row.get(6)?,
                updated_at: row.get(7)?,
            })
        }).optional()
    }
}

// --- Sync log ---

pub fn insert_sync_log(conn: &Connection, user_id: Option<i64>, project_id: Option<i64>, status: &str, error_msg: Option<&str>, items: i64) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO sync_log (user_id, project_id, status, error_message, items_synced, started_at, finished_at) VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), datetime('now'))",
        params![user_id, project_id, status, error_msg, items],
    )?;
    Ok(())
}

pub fn get_last_sync(conn: &Connection) -> rusqlite::Result<Option<String>> {
    conn.query_row(
        "SELECT finished_at FROM sync_log WHERE status = 'success' ORDER BY id DESC LIMIT 1",
        [],
        |row| row.get(0),
    ).optional()
}

// --- Score computation helpers ---

pub fn compute_lines_score(conn: &Connection, user_id: i64, project_id: Option<i64>) -> rusqlite::Result<i64> {
    let sql = if project_id.is_some() {
        "SELECT COALESCE(SUM(CAST(additions AS REAL) + CAST(deletions AS REAL) * 0.5), 0) FROM pull_requests WHERE user_id = ?1 AND project_id = ?2 AND (state = 'merged' OR merged_at IS NOT NULL)"
    } else {
        "SELECT COALESCE(SUM(CAST(additions AS REAL) + CAST(deletions AS REAL) * 0.5), 0) FROM pull_requests WHERE user_id = ?1 AND (state = 'merged' OR merged_at IS NOT NULL)"
    };

    if let Some(pid) = project_id {
        conn.query_row(sql, params![user_id, pid], |row| row.get::<_, f64>(0))
            .map(|v| v as i64)
    } else {
        conn.query_row(sql, params![user_id], |row| row.get::<_, f64>(0))
            .map(|v| v as i64)
    }
}

pub fn compute_comments_score(conn: &Connection, user_id: i64, project_id: Option<i64>) -> rusqlite::Result<i64> {
    let (issue_comments, pr_comments, issues_opened, pr_inline_comments) = if let Some(pid) = project_id {
        let ic: i64 = conn.query_row(
            "SELECT COUNT(*) FROM comments WHERE user_id = ?1 AND project_id = ?2 AND target_type = 'issue'",
            params![user_id, pid], |row| row.get(0))?;
        let pc: i64 = conn.query_row(
            "SELECT COUNT(*) FROM comments WHERE user_id = ?1 AND project_id = ?2 AND target_type = 'pr'",
            params![user_id, pid], |row| row.get(0))?;
        let io: i64 = conn.query_row(
            "SELECT COUNT(*) FROM issues WHERE user_id = ?1 AND project_id = ?2",
            params![user_id, pid], |row| row.get(0))?;
        // PR inline comments count from comments_count field on PRs
        let pic: i64 = conn.query_row(
            "SELECT COALESCE(SUM(comments_count), 0) FROM pull_requests WHERE user_id = ?1 AND project_id = ?2",
            params![user_id, pid], |row| row.get(0))?;
        (ic, pc, io, pic)
    } else {
        let ic: i64 = conn.query_row(
            "SELECT COUNT(*) FROM comments WHERE user_id = ?1 AND target_type = 'issue'",
            params![user_id], |row| row.get(0))?;
        let pc: i64 = conn.query_row(
            "SELECT COUNT(*) FROM comments WHERE user_id = ?1 AND target_type = 'pr'",
            params![user_id], |row| row.get(0))?;
        let io: i64 = conn.query_row(
            "SELECT COUNT(*) FROM issues WHERE user_id = ?1",
            params![user_id], |row| row.get(0))?;
        let pic: i64 = conn.query_row(
            "SELECT COALESCE(SUM(comments_count), 0) FROM pull_requests WHERE user_id = ?1",
            params![user_id], |row| row.get(0))?;
        (ic, pc, io, pic)
    };
    // issue comments ×2, PR review comments ×3, issues opened ×5, PR inline review ×3
    Ok(issue_comments * 2 + pr_comments * 3 + issues_opened * 5 + pr_inline_comments * 3)
}

pub fn compute_bonus_total(conn: &Connection, user_id: i64, project_id: Option<i64>) -> rusqlite::Result<i64> {
    let sql = if project_id.is_some() {
        "SELECT COALESCE(SUM(points), 0) FROM bonus_points WHERE user_id = ?1 AND (project_id = ?2 OR project_id IS NULL)"
    } else {
        "SELECT COALESCE(SUM(points), 0) FROM bonus_points WHERE user_id = ?1"
    };

    if let Some(pid) = project_id {
        conn.query_row(sql, params![user_id, pid], |row| row.get(0))
    } else {
        conn.query_row(sql, params![user_id], |row| row.get(0))
    }
}

// --- Session / Auth queries ---

pub fn create_session(conn: &Connection, user_id: i64, token: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO sessions (user_id, token, expires_at) VALUES (?1, ?2, datetime('now', '+7 days'))",
        params![user_id, token],
    )?;
    Ok(())
}

pub fn validate_session(conn: &Connection, token: &str) -> rusqlite::Result<Option<User>> {
    conn.query_row(
        "SELECT u.id, u.github_username, u.github_pat, u.display_name, u.avatar_url, u.is_maintainer, u.created_at, u.updated_at
         FROM sessions s JOIN users u ON u.id = s.user_id
         WHERE s.token = ?1 AND s.expires_at > datetime('now')",
        params![token],
        |row| Ok(User {
            id: row.get(0)?,
            github_username: row.get(1)?,
            github_pat: row.get(2)?,
            display_name: row.get(3)?,
            avatar_url: row.get(4)?,
            is_maintainer: row.get::<_, i32>(5)? != 0,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        }),
    ).optional()
}

pub fn delete_session(conn: &Connection, token: &str) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM sessions WHERE token = ?1", params![token])?;
    Ok(())
}

pub fn cleanup_expired_sessions(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM sessions WHERE expires_at <= datetime('now')", [])?;
    Ok(())
}

pub fn get_user_by_username(conn: &Connection, username: &str) -> rusqlite::Result<Option<User>> {
    conn.query_row(
        "SELECT id, github_username, github_pat, display_name, avatar_url, is_maintainer, created_at, updated_at FROM users WHERE github_username = ?1",
        params![username],
        |row| Ok(User {
            id: row.get(0)?,
            github_username: row.get(1)?,
            github_pat: row.get(2)?,
            display_name: row.get(3)?,
            avatar_url: row.get(4)?,
            is_maintainer: row.get::<_, i32>(5)? != 0,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        }),
    ).optional()
}

pub fn set_user_password(conn: &Connection, user_id: i64, password_hash: &str) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE users SET password_hash = ?1, updated_at = datetime('now') WHERE id = ?2",
        params![password_hash, user_id],
    )?;
    Ok(())
}

pub fn get_user_password_hash(conn: &Connection, user_id: i64) -> rusqlite::Result<Option<String>> {
    conn.query_row(
        "SELECT password_hash FROM users WHERE id = ?1",
        params![user_id],
        |row| row.get(0),
    ).optional()
}

// --- Project hotness (last 30 days contribution score sum) ---

pub fn get_project_hotness(conn: &Connection, project_id: i64) -> rusqlite::Result<i64> {
    // Lines from merged PRs in last 30 days
    let lines: f64 = conn.query_row(
        "SELECT COALESCE(SUM(CAST(additions AS REAL) + CAST(deletions AS REAL) * 0.5), 0)
         FROM pull_requests
         WHERE project_id = ?1 AND (state = 'merged' OR merged_at IS NOT NULL)
         AND created_at >= datetime('now', '-30 days')",
        params![project_id],
        |row| row.get(0),
    )?;

    // Comments in last 30 days
    let comments: i64 = conn.query_row(
        "SELECT COUNT(*) FROM comments WHERE project_id = ?1 AND created_at >= datetime('now', '-30 days')",
        params![project_id],
        |row| row.get(0),
    )?;

    // Issues opened in last 30 days
    let issues: i64 = conn.query_row(
        "SELECT COUNT(*) FROM issues WHERE project_id = ?1 AND created_at >= datetime('now', '-30 days')",
        params![project_id],
        |row| row.get(0),
    )?;

    // Bonus points in last 30 days
    let bonus: i64 = conn.query_row(
        "SELECT COALESCE(SUM(points), 0) FROM bonus_points WHERE project_id = ?1 AND created_at >= datetime('now', '-30 days')",
        params![project_id],
        |row| row.get(0),
    )?;

    Ok(lines as i64 + comments * 3 + issues * 5 + bonus)
}

pub fn list_projects_by_hotness(conn: &Connection) -> rusqlite::Result<Vec<(Project, i64)>> {
    let projects = list_projects(conn)?;
    let mut results: Vec<(Project, i64)> = projects
        .into_iter()
        .map(|p| {
            let hotness = get_project_hotness(conn, p.id).unwrap_or(0);
            (p, hotness)
        })
        .collect();
    results.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(results)
}

