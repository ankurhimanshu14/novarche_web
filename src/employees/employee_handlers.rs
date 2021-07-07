#[path = "../schema.rs"]
mod schema;

#[path = "../connection.rs"]
mod connection;

use super::employee_models::{NewEmployee, Employee, Title};
use crate::schema::employees::dsl::*;
use crate::connection::Pool;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputEmployee {
    pub employee_id: String,
    pub title: Title,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub uidai: i64,
    pub pan: String,
    pub uan: i64
}

// Handler for GET /employees
pub async fn get_employees(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_employees(db))
        .await
        .map(|employee| HttpResponse::Ok().json(employee))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for GET /employees/{id}
pub async fn get_employee_by_id(
    db: web::Data<Pool>,
    e_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_employee_by_id(db, e_id.into_inner()))
            .await
            .map(|employee| HttpResponse::Ok().json(employee))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for POST /employees
pub async fn add_employee(
    db: web::Data<Pool>,
    item: web::Json<InputEmployee>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_employee(db, item))
        .await
        .map(|employee| HttpResponse::Created().json(employee))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for DELETE /employees/{id}
pub async fn delete_employee(
    db: web::Data<Pool>,
    e_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_employee(db, e_id.into_inner()))
            .await
            .map(|employee| HttpResponse::Ok().json(employee))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}


fn get_all_employees(pool: web::Data<Pool>) -> Result<Vec<Employee>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = employees.load::<Employee>(&conn)?;
    Ok(items)
}

fn db_get_employee_by_id(pool: web::Data<Pool>, e_id: i32) -> Result<Employee, diesel::result::Error> {
    let conn = pool.get().unwrap();
    employees.find(e_id).get_result::<Employee>(&conn)
}

fn add_single_employee(
    db: web::Data<Pool>,
    item: web::Json<InputEmployee>,
) -> Result<Employee, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_employee = NewEmployee {
        employee_id: &item.employee_id,
        title: &item.title,
        first_name: &item.first_name,
        middle_name: match &item.middle_name.as_ref().unwrap().len() {
            0 => None,
            _ => Some(&item.middle_name.as_ref().unwrap())
        },
        last_name: &item.last_name,
        uidai: &item.uidai,
        pan: &item.pan,
        uan: &item.uan
    };
    let res = insert_into(employees).values(&new_employee).get_result(&conn)?;
    Ok(res)
}

fn delete_single_employee(db: web::Data<Pool>, e_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(employees.find(e_id)).execute(&conn)?;
    Ok(count)
}