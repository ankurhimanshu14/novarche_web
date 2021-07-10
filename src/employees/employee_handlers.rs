#[path = "../schema.rs"]
mod schema;

#[path = "../utils.rs"]
mod utils;

use super::employee_models::{NewEmployee, Employee};
use crate::schema::employees::dsl::*;
use crate::utils::Pool;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputEmployee {
    pub person_id: i32,
    pub employee_id: String,
    pub dept_id: String,
    pub salary: i32,
    pub doj: String,
    pub created_by: String
}

//Handler for GET /employees
pub async fn get_employees(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_employees(db))
    .await
    .map(|employee| HttpResponse::Ok().json(employee))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for GET /employees/{id}
pub async fn get_employee_by_id(
    db: web::Data<Pool>,
    e_id: web::Path<String>,
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
    e_id: web::Path<String>,
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

fn db_get_employee_by_id(pool: web::Data<Pool>, e_id: String) -> Result<Employee, diesel::result::Error> {
    let conn = pool.get().unwrap();
    employees.find(e_id).get_result::<Employee>(&conn)
}

fn add_single_employee(
    pool: web::Data<Pool>,
    item: web::Json<InputEmployee>,
) -> Result<Employee, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let new_employee = NewEmployee {
        person_id: &item.person_id,
        employee_id: &item.employee_id,
        dept_id: &item.dept_id,
        salary: &item.salary,
        doj: chrono::NaiveDate::parse_from_str(&item.doj, "%d-%m-%Y").unwrap(),
        created_by: &item.created_by
    };

    let res = insert_into(employees).values(&new_employee).get_result(&conn)?;
    Ok(res)
}

fn delete_single_employee(pool: web::Data<Pool>, e_id: String) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let count = delete(employees.find(e_id)).execute(&conn)?;
    Ok(count)

}