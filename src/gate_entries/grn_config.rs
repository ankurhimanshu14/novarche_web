use actix_web:: web;

use super::grn_handlers::*;

pub fn grn_config(cfg: &mut web::ServiceConfig) {

    let scope = web::scope("/grns")
            .route("", web::get().to(get_grns))
            .route("/{grn_id}", web::get().to(get_grn_by_id))
            .route("", web::post().to(add_grn))
            .route("/{grn_id}", web::delete().to(delete_grn));

    cfg.service(scope);
}