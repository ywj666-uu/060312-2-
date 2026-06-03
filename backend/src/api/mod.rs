pub mod users;
pub mod projects;
pub mod contributions;
pub mod leaderboard;
pub mod admin;
pub mod sync;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(users::configure)
            .configure(projects::configure)
            .configure(contributions::configure)
            .configure(leaderboard::configure)
            .configure(admin::configure)
            .configure(sync::configure)
    );
}
