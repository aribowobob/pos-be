use actix_web::web;
use log::info;
use crate::handlers::auth::{google_login, logout};

pub fn configure(cfg: &mut web::ServiceConfig) {
    info!("Configuring auth routes");
    cfg.service(
        web::scope("/auth")
            .route("/google", web::post().to(google_login))
            .route("/logout", web::post().to(logout))
    );
}
