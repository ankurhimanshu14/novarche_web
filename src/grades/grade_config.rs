use actix_web::{ get, web, App, HttpRequest };

use super::grade_handlers;

pub fn grade_config(cfg: &mut web::ServiceConfig) {

    let scope = web::scope("/grades")
            .route("", web::get().to(grade_handlers::get_grades))
            .route("/{id}", web::get().to(grade_handlers::get_grade_by_id))
            .route("", web::post().to(grade_handlers::add_grade))
            .route("/{id}", web::delete().to(grade_handlers::delete_grade));

    cfg.service(scope);
}