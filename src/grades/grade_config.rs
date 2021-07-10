use actix_web:: web;

use super::grade_handlers::*;

pub fn grade_config(cfg: &mut web::ServiceConfig) {

    let scope = web::scope("/grades")
            .route("", web::get().to(get_grades))
            .route("/{grade_id}", web::get().to(get_grade_by_id))
            .route("", web::post().to(add_grade))
            .route("/{grade_id}", web::delete().to(delete_grade));

    cfg.service(scope);
}