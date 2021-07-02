use actix_web::{ Responder, HttpResponse };

pub async fn get_grades() -> impl Responder {
    HttpResponse::Ok().body("Hello from get grades")
}

pub async fn get_grade_by_id() -> impl Responder {
    HttpResponse::Ok().body("Hello from get grade by id")
}

pub async fn add_grade() -> impl Responder {
    HttpResponse::Ok().body("Hello from add grade")
}

pub async fn delete_grade() -> impl Responder {
    HttpResponse::Ok().body("Hello from delete grade")
}