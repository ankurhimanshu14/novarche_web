#[path = "../schema.rs"]
mod schema;

#[path = "../connection.rs"]
mod connection;

use bcrypt::{ DEFAULT_COST, hash, verify };
use crate::schema::users::dsl::*;
use crate::connection::Pool;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::dsl::{delete, insert_into};
use actix_web::{web, Error, HttpResponse, Responder};
use actix_identity::Identity;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

use super::user_models::{NewUser, User, LoginUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub hash: String
}

// Handler for GET /users
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
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

// Handler for POST /users
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_user(db, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for DELETE /users/{id}
pub async fn delete_user(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_user(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

// Handler for GET /users/logout
pub async fn logout(session_id: Identity) -> impl Responder {
    session_id.forget();
    HttpResponse::Ok().body("Logged out.")
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

fn add_single_user(
    pool: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let new_user = NewUser {
        first_name: &item.first_name,
        middle_name: match &item.middle_name.as_ref().unwrap().len() {
            0 => None,
            _ => Some(&item.middle_name.as_ref().unwrap())
        },
        last_name: &item.last_name,
        email: &item.email,
        username: &item.username,
        hash: &hash(&item.hash, DEFAULT_COST).unwrap()
    };
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}

fn delete_single_user(pool: web::Data<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}

fn login_user(pool: web::Data<Pool>, login_user: LoginUser) -> Result<bool, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let user = users.filter(username.eq(&login_user.username)).first::<User>(&conn);

    match user {
        Ok(u) => {
            if verify(&login_user.password, &u.hash).unwrap() && &u.status == "ACTIVE" {                
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