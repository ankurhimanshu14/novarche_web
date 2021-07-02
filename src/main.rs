use actix_web::{ web, App, HttpServer };

mod config;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .service(
            web::scope("/api/v1/routes")
            .configure(config::config)
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}