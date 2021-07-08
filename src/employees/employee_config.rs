use actix_web:: web;

use super::employee_handlers::*;

pub fn employee_config(cfg: &mut web::ServiceConfig) {

    let scope = web::scope("/employees")
            .route("", web::get().to(get_employees))
            .route("/{employee_id}", web::get().to(get_employee_by_id))
            .route("", web::post().to(add_employee))
            .route("/{employee_id}", web::delete().to(delete_employee));

    cfg.service(scope);
}