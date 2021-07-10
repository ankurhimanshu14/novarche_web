#[path = "../schema.rs"]
mod schema;

#[path = "../connection.rs"]
mod connection;

use super::person_models::{NewPerson, Person};
use crate::schema::persons::dsl::*;
use crate::connection::Pool;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputPerson {
    pub title: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub gender: String,
    pub dob: String,
    pub address: Option<String>,
    pub email: Option<String>,
    pub uidai: i64,
    pub uan: i64,
    pub pan: Option<String>,
    pub created_by: String
}

//Handler for GET /persons
pub async fn get_persons(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_persons(db))
    .await
    .map(|person| HttpResponse::Ok().json(person))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for GET /persons/{id}
pub async fn get_person_by_id(
    db: web::Data<Pool>,
    person_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_person_by_id(db, person_id.into_inner()))
            .await
            .map(|person| HttpResponse::Ok().json(person))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for POST /persons
pub async fn add_person(
    db: web::Data<Pool>,
    item: web::Json<InputPerson>,
) -> Result<HttpResponse, Error> {    
    Ok(web::block(move || add_single_person(db, item))
        .await
        .map(|person| HttpResponse::Created().json(person))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for DELETE /persons/{id}
pub async fn delete_person(
    db: web::Data<Pool>,
    person_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_person(db, person_id.into_inner()))
            .await
            .map(|person| HttpResponse::Ok().json(person))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn get_all_persons(pool: web::Data<Pool>) -> Result<Vec<Person>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = persons.load::<Person>(&conn)?;
    Ok(items)
}

fn db_get_person_by_id(pool: web::Data<Pool>, person_id: i32) -> Result<Person, diesel::result::Error> {
    let conn = pool.get().unwrap();
    persons.find(person_id).get_result::<Person>(&conn)
}

fn add_single_person(
    pool: web::Data<Pool>,
    item: web::Json<InputPerson>,
) -> Result<Person, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let new_person = NewPerson {
        title: &item.title,
        first_name: &item.first_name,
        middle_name: match &item.middle_name.as_ref().unwrap().is_empty() {
            true => None,
            false => Some(&item.middle_name.as_ref().unwrap())
        },
        last_name: &item.last_name,
        gender: &item.gender,
        dob: chrono::NaiveDate::parse_from_str(&item.dob, "%d-%m-%Y").unwrap(),
        address: match &item.address.as_ref().unwrap().is_empty() {
            true => None,
            false => Some(&item.address.as_ref().unwrap())
        },
        email: match &item.email.as_ref().unwrap().is_empty() {
            true => None,
            false => Some(&item.email.as_ref().unwrap())
        },
        uidai: &item.uidai,
        uan: &item.uan,
        pan: match &item.pan.as_ref().unwrap().is_empty() {
            true => None,
            false => Some(&item.pan.as_ref().unwrap())
        },
        created_by: &item.created_by
    };

    let res = insert_into(persons).values(&new_person).get_result(&conn)?;
    Ok(res)
}

fn delete_single_person(pool: web::Data<Pool>, person_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let count = delete(persons.find(person_id)).execute(&conn)?;
    Ok(count)

}