use actix_web:: web;

use super::auth_handlers;

pub fn auth_config(cfg: &mut web::ServiceConfig) {

    let scope = web::scope("/auth")
            .route("/login", web::post().to(auth_handlers::process_login))
            .route("/logout", web::to(auth_handlers::logout));

    cfg.service(scope);
}