use std::sync::Mutex;
use rusqlite::Connection;
use crate::db::queries;

pub fn recalculate_all_scores(db: &Mutex<Connection>) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let users = queries::list_users(&conn).map_err(|e| e.to_string())?;
    let projects = queries::list_projects(&conn).map_err(|e| e.to_string())?;

    for user in &users {
        let lines = queries::compute_lines_score(&conn, user.id, None).unwrap_or(0);
        let comments = queries::compute_comments_score(&conn, user.id, None).unwrap_or(0);
        let bonus = queries::compute_bonus_total(&conn, user.id, None).unwrap_or(0);
        let _ = queries::upsert_score(&conn, user.id, None, lines, comments, bonus);

        for project in &projects {
            let lines = queries::compute_lines_score(&conn, user.id, Some(project.id)).unwrap_or(0);
            let comments = queries::compute_comments_score(&conn, user.id, Some(project.id)).unwrap_or(0);
            let bonus = queries::compute_bonus_total(&conn, user.id, Some(project.id)).unwrap_or(0);
            let _ = queries::upsert_score(&conn, user.id, Some(project.id), lines, comments, bonus);
        }
    }
    Ok(())
}
