use actix_web::web;

use crate::handlers::user::get_user;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/get-user", web::get().to(get_user)),
    );
}
