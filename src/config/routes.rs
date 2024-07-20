use actix_web::web;
use crate::api::*;
pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
        .service(
            web::resource("/login").route(web::get().to(user::login))
        )
    );
}