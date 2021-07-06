use actix_web:: web;

use super::user_handlers;

pub fn user_config(cfg: &mut web::ServiceConfig) {

    let scope = web::scope("/users")
            .route("", web::get().to(user_handlers::get_users))
            .route("", web::post().to(user_handlers::add_user))
            .route("/{id}", web::delete().to(user_handlers::delete_user));

    cfg.service(scope);
}