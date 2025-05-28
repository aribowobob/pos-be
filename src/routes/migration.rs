use actix_web::{web, HttpResponse, Responder, get};
use crate::handlers::migration::db_migration;
use crate::middleware::skip_auth::SkipAuth;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/db-migration")
            .wrap(SkipAuth)
            .route(web::get().to(db_migration))
    );
}
