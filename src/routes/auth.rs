use crate::handlers::auth::google_auth;
use actix_web::web;

// Configure auth routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").route("/google", web::post().to(google_auth)));
}
