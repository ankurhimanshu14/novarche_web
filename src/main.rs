use actix_web::{ HttpServer, App, middleware::Logger, web };
use dotenv;
use env_logger;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate num;

mod utils;
mod schema;
mod person;
mod employees;
mod users;
mod grades;
mod gate_entries;
mod approvals;

use crate::person::person_config::person_config;
use crate::employees::employee_config::employee_config;
use crate::users::user_config::user_config;
use crate::grades::grade_config::grade_config;
use crate::gate_entries::grn_config::grn_config;
use crate::approvals::approval_config::approval_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    let pool = utils::init_pool();
    
    let host = dotenv::var("HOST").unwrap();
    let port = dotenv::var("PORT").unwrap();

    HttpServer::new(move || {
        App::new()
        .wrap(utils::cors())
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        .data(pool.clone())
        .service(
            web::scope("/api/v1/routes")
            .configure(person_config)
            .configure(employee_config)
            .configure(user_config)
            .configure(grade_config)
            .configure(grn_config)
            .configure(approval_config)
        )
    })
    .workers(10)
    .bind(format!("{}:{}", &host, &port))?
    .run()
    .await
}