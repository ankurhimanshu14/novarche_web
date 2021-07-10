use actix_web:: web;

use super::person_handlers::*;

pub fn person_config(cfg: &mut web::ServiceConfig) {

    let scope = web::scope("/persons")
            .route("", web::get().to(get_persons))
            .route("/{person_id}", web::get().to(get_person_by_id))
            .route("", web::post().to(add_person))
            .route("/{person_id}", web::delete().to(delete_person));

    cfg.service(scope);
}