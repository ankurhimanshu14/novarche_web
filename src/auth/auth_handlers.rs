#[path = "../schema.rs"]
mod schema;

#[path = "../connection.rs"]
mod connection;

#[path = "../users/user_models.rs"]
mod users_models;

use bcrypt::verify;
use crate::schema::users::dsl::*;
use crate::connection::Pool;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use actix_web::{web, Error, HttpResponse, Responder};
use actix_identity::Identity;
use serde::{Deserialize, Serialize};

use super::auth_models::LoginUser;

use crate::users::user_models::{User, Userstatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub hash: String
}

//Handler for POST /users/login
pub async fn process_login(
    db: web::Data<Pool>,
    login: web::Json<LoginUser>,
    session_id: Identity
) -> Result<HttpResponse, Error> {

    let session_token = String::from(login.username.clone());
                
    session_id.remember(session_token);

    if let Some(_) = session_id.identity() {
        Ok(
            web::block(move || login_user(db, login.into_inner()))
                .await
                .map(|user| {
                    match user {
                        true => HttpResponse::Ok().body(format!("Logged In")),
                        false => HttpResponse::Ok().body(format!("Wrong Credentials")),
                    }
                })
                .map_err(|_| HttpResponse::InternalServerError())?
        )
    } else {
        Ok(HttpResponse::Ok().body("Login to proceed"))
    }
}

// Handler for GET /users/logout
pub async fn logout(session_id: Identity) -> impl Responder {
    session_id.forget();
    HttpResponse::Ok().body("Logged out.")
}

fn login_user(pool: web::Data<Pool>, login_user: LoginUser) -> Result<bool, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let user = users.filter(username.eq(&login_user.username)).first::<User>(&conn);

    match user {
        Ok(u) => {
            if verify(&login_user.password, &u.hash).unwrap() && &u.status == Userstatus::Active {                
                Ok(true)
            
            } else {
                Ok(false)
            }
        },
        Err(e) => {
            println!("{}: {:?}", &login_user.username, e);
            Ok(false)
        }
    }
}