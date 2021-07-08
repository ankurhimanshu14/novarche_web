use actix_web::{ HttpServer, App, web, Responder, HttpResponse };
use dotenv;

#[macro_use]
extern crate diesel;
extern crate chrono;

mod connection;
mod schema;
mod employees;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello Ankita!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = connection::init_pool();
    
    let host = dotenv::var("HOST").unwrap();
    let port = dotenv::var("PORT").unwrap();

    HttpServer::new(move || {
        App::new()
        .data(pool.clone())
        .route("/hello", web::get().to(index))
    })
    .bind(format!("{}:{}", &host, &port))?
    .run()
    .await
}