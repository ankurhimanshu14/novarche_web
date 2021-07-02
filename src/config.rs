use actix_web::{ web };

use crate::users::handlers::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
        .route("", web::get().to(get_users))
        .route("/{id}", web::get().to(get_user_by_id))
        .route("", web::post().to(add_user))
        .route("/{id}", web::delete().to(delete_user))
    );
}