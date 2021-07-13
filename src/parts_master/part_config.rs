use actix_web:: web;

use super::part_handlers::*;

pub fn part_config(cfg: &mut web::ServiceConfig) {

    let scope = web::scope("/parts")
            .route("", web::get().to(get_parts))
            .route("/{part_no}", web::get().to(get_part_by_no))
            .route("", web::post().to(add_part))
            .route("/{part_no}", web::delete().to(delete_part));

    cfg.service(scope);
}