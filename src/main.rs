use actix_web::{ web, App, HttpServer };

mod users;
mod grades;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
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