use actix_web::{ HttpServer, App, web };
use dotenv;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate num;

mod connection;
mod schema;
mod employees;

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
            .configure(employees::employee_config::employee_config)
        )
    })
    .workers(10)
    .bind(format!("{}:{}", &host, &port))?
    .run()
    .await
}