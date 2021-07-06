use actix_web:: web;

use super::employee_handlers;

pub fn employee_config(cfg: &mut web::ServiceConfig) {

    let scope = web::scope("/employees")
            .route("", web::get().to(employee_handlers::get_employees))
            .route("", web::post().to(employee_handlers::add_employee))
            .route("/{id}", web::delete().to(employee_handlers::delete_employee));

    cfg.service(scope);
}