#[path = "../schema.rs"]
mod schema;

#[path = "../utils.rs"]
mod utils;

use super::part_models::{NewPart, Part};
use crate::schema::parts::dsl::*;
use crate::utils::Pool;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputPart {
    pub part_code: String,
    pub part_no: i32,
    pub part_name: String,
    pub material: String,
    pub forging_wt: f32,
    pub cut_wt: f32,
    pub created_by: String
}

//Handler for GET /parts
pub async fn get_parts(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_parts(db))
    .await
    .map(|part| HttpResponse::Ok().json(part))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for GET /parts/{id}
pub async fn get_part_by_no(
    db: web::Data<Pool>,
    p_no: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_part_by_no(db, p_no.into_inner()))
            .await
            .map(|part| HttpResponse::Ok().json(part))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for POST /parts
pub async fn add_part(
    db: web::Data<Pool>,
    item: web::Json<InputPart>,
) -> Result<HttpResponse, Error> {    
    Ok(web::block(move || add_single_part(db, item))
        .await
        .map(|part| HttpResponse::Created().json(part))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for DELETE /parts/{id}
pub async fn delete_part(
    db: web::Data<Pool>,
    p_no: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_part(db, p_no.into_inner()))
            .await
            .map(|part| HttpResponse::Ok().json(part))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn get_all_parts(pool: web::Data<Pool>) -> Result<Vec<Part>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = parts.load::<Part>(&conn)?;
    Ok(items)
}

fn db_get_part_by_no(pool: web::Data<Pool>, p_no: i32) -> Result<Part, diesel::result::Error> {
    let conn = pool.get().unwrap();
    parts.find(p_no).get_result::<Part>(&conn)
}

fn add_single_part(
    pool: web::Data<Pool>,
    item: web::Json<InputPart>,
) -> Result<Part, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let new_part = NewPart {
        part_code: &item.part_code,
        part_no: &item.part_no,
        part_name: &item.part_name,
        material: &item.material,
        forging_wt: &item.forging_wt,
        cut_wt: &item.cut_wt,
        created_by: &item.created_by,
    };

    let res = insert_into(parts).values(&new_part).get_result(&conn)?;
    Ok(res)
}

fn delete_single_part(pool: web::Data<Pool>, p_no: i32) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let count = delete(parts.find(p_no)).execute(&conn)?;
    Ok(count)

}