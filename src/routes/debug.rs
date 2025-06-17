use actix_web::web;
use crate::handlers::debug::debug_env;
use log::info;

pub fn configure(cfg: &mut web::ServiceConfig) {
    info!("Configuring debug routes");
    cfg.service(
        web::scope("/debug")
            .route("/env", web::get().to(debug_env))
    );
}
