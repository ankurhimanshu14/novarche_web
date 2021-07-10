use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use actix_cors::Cors;
use actix_web::http;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    Pool::new(manager).expect("Failed to create pool")
}

pub fn cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:4200")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "PATCH"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}