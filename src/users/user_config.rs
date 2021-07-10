use actix_web:: web;

use super::user_handlers::*;

pub fn user_config(cfg: &mut web::ServiceConfig) {

    let scope = web::scope("/users")
            .route("", web::get().to(get_users))
            .route("/{user_id}", web::get().to(get_user_by_id))
            .route("", web::post().to(add_user))
            .route("/{user_id}", web::delete().to(delete_user));

    cfg.service(scope);
}