#[path = "../schema.rs"]
mod schema;

#[path = "../connection.rs"]
mod connection;

use bcrypt::{ DEFAULT_COST, hash };
use super::user_models::{NewUser, User};
use crate::schema::users::dsl::*;
use crate::connection::Pool;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub employee_id: String,
    pub username: String,
    password: String,
    pub created_by: String
}

//Handler for GET /users
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
    .await
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for GET /users/{id}
pub async fn get_user_by_id(
    db: web::Data<Pool>,
    u_name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_user_by_id(db, u_name.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
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
    u_name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_user(db, u_name.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

fn db_get_user_by_id(pool: web::Data<Pool>, u_name: String) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(u_name).get_result::<User>(&conn)
}

fn add_single_user(
    pool: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let new_user = NewUser {
        employee_id: &item.employee_id,
        username: &item.username,
        password: &hash(&item.password, DEFAULT_COST).unwrap(),
        created_by: &item.created_by
    };

    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}

fn delete_single_user(pool: web::Data<Pool>, u_name: String) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let count = delete(users.find(u_name)).execute(&conn)?;
    Ok(count)

}