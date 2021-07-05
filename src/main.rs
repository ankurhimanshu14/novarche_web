use actix_web::{ web, App, HttpServer };

mod users;
mod grades;
mod connection;
mod schema;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate serde;

use connection::init_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = init_pool();

    HttpServer::new(move || {
        App::new()
        .data(pool.clone())
        .service(
            web::scope("/api/v1/routes")
            .configure(users::user_config::user_config)
            .configure(grades::grade_config::grade_config)
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}