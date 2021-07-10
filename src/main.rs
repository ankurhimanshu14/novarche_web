use actix_web::{ HttpServer, App, web };
use dotenv;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate num;

mod connection;
mod schema;
mod person;
mod employees;
mod users;

use crate::person::person_config::person_config;
use crate::employees::employee_config::employee_config;
use crate::users::user_config::user_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = connection::init_pool();
    
    let host = dotenv::var("HOST").unwrap();
    let port = dotenv::var("PORT").unwrap();

    HttpServer::new(move || {
        App::new()
        .data(pool.clone())
        .service(
            web::scope("/api/v1/routes")
            .configure(person_config)
            .configure(employee_config)
            .configure(user_config)
        )
    })
    .workers(10)
    .bind(format!("{}:{}", &host, &port))?
    .run()
    .await
}