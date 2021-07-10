#[path = "../schema.rs"]
mod schema;

#[path = "../connection.rs"]
mod connection;

use bcrypt::{ DEFAULT_COST, hash };
use super::grade_models::{NewGrade, Grade};
use crate::schema::grades::dsl::*;
use crate::connection::Pool;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputGrade {
    pub grade_name: String,
    pub size: i32,
    pub section: String,
    pub created_by: String
}

//Handler for GET /grades
pub async fn get_grades(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_grades(db))
    .await
    .map(|grade| HttpResponse::Ok().json(grade))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for GET /grades/{id}
pub async fn get_grade_by_id(
    db: web::Data<Pool>,
    g_name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_grade_by_id(db, g_name.into_inner()))
            .await
            .map(|grade| HttpResponse::Ok().json(grade))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for POST /grades
pub async fn add_grade(
    db: web::Data<Pool>,
    item: web::Json<InputGrade>,
) -> Result<HttpResponse, Error> {    
    Ok(web::block(move || add_single_grade(db, item))
        .await
        .map(|grade| HttpResponse::Created().json(grade))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for DELETE /grades/{id}
pub async fn delete_grade(
    db: web::Data<Pool>,
    g_name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_grade(db, g_name.into_inner()))
            .await
            .map(|grade| HttpResponse::Ok().json(grade))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn get_all_grades(pool: web::Data<Pool>) -> Result<Vec<Grade>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = grades.load::<Grade>(&conn)?;
    Ok(items)
}

fn db_get_grade_by_id(pool: web::Data<Pool>, g_name: String) -> Result<Grade, diesel::result::Error> {
    let conn = pool.get().unwrap();
    grades.find(g_name).get_result::<Grade>(&conn)
}

fn add_single_grade(
    pool: web::Data<Pool>,
    item: web::Json<InputGrade>,
) -> Result<Grade, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let new_grade = NewGrade {
        grade_name: &item.grade_name,
        size: &item.size,
        section: &item.section,
        created_by: &item.created_by
    };

    let res = insert_into(grades).values(&new_grade).get_result(&conn)?;
    Ok(res)
}

fn delete_single_grade(pool: web::Data<Pool>, g_name: String) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let count = delete(grades.find(g_name)).execute(&conn)?;
    Ok(count)

}