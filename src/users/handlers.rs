use actix_web::{ Responder, HttpResponse };

pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Hello from get users")
}

pub async fn get_user_by_id() -> impl Responder {
    HttpResponse::Ok().body("Hello from get user by id")
}

pub async fn add_user() -> impl Responder {
    HttpResponse::Ok().body("Hello from add user")
}

pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("Hello from delete user")
}