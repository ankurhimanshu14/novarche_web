use actix_web:: web;

use super::approval_handlers::*;

pub fn approval_config(cfg: &mut web::ServiceConfig) {

    let scope = web::scope("/approvals")
            .route("", web::get().to(get_approvals))
            .route("/{approval_id}", web::get().to(get_approval_by_id))
            .route("", web::post().to(add_approval))
            .route("/{approval_id}", web::delete().to(delete_approval));

    cfg.service(scope);
}