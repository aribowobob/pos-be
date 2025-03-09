use crate::handlers::user::get_user;
use actix_web::web;

// Configure user routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").route("/get-user", web::get().to(get_user)));
}
