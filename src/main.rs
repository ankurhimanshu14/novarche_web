use actix_web::{ web, App, HttpServer };
use actix_identity::{ IdentityService, CookieIdentityPolicy };

mod users;
mod auth;
mod grades;
mod connection;
mod schema;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate serde;

use connection::{ init_pool };

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = init_pool();

    HttpServer::new(move || {
        App::new()
        .wrap(IdentityService::new(
            CookieIdentityPolicy::new(&[0; 32])
            .name("auth-cookie")
            .secure(false)
        ))
        .data(pool.clone())
        .service(
            web::scope("/api/v1/routes")
            .configure(auth::auth_config::auth_config)
        )
        .service(
            web::scope("/api/v1/routes/{auth-token}")
            .configure(users::user_config::user_config)
            .configure(grades::grade_config::grade_config)
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}