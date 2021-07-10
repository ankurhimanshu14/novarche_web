#[path = "../schema.rs"]
mod schema;

#[path = "../utils.rs"]
mod utils;

use super::grn_models::{NewGRN, GRN};
use crate::schema::grns::dsl::*;
use crate::utils::Pool;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputGRN {
    pub grn: i64,
    pub challan_no: i64,
    pub challan_date: String,
    pub grade_name: String,
    pub size: i32,
    pub section: String,
    pub heat_no: String,
    pub heat_code: Option<String>,
    pub received_qty: i32,
    pub created_by: String,
}

//Handler for GET /GRNs
pub async fn get_grns(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_grns(db))
    .await
    .map(|gate_entry| HttpResponse::Ok().json(gate_entry))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for GET /GRNs/{id}
pub async fn get_grn_by_id(
    db: web::Data<Pool>,
    gr_no: web::Path<i64>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_grn_by_id(db, gr_no.into_inner()))
            .await
            .map(|gate_entry| HttpResponse::Ok().json(gate_entry))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for POST /GRNs
pub async fn add_grn(
    db: web::Data<Pool>,
    item: web::Json<InputGRN>,
) -> Result<HttpResponse, Error> {    
    Ok(web::block(move || add_single_grn(db, item))
        .await
        .map(|gate_entry| HttpResponse::Created().json(gate_entry))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for DELETE /GRNs/{id}
pub async fn delete_grn(
    db: web::Data<Pool>,
    gr_no: web::Path<i64>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_grn(db, gr_no.into_inner()))
            .await
            .map(|gate_entry| HttpResponse::Ok().json(gate_entry))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn get_all_grns(pool: web::Data<Pool>) -> Result<Vec<GRN>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = grns.load::<GRN>(&conn)?;
    Ok(items)
}

fn db_get_grn_by_id(pool: web::Data<Pool>, gr_no: i64) -> Result<GRN, diesel::result::Error> {
    let conn = pool.get().unwrap();
    grns.find(gr_no).get_result::<GRN>(&conn)
}

fn add_single_grn(
    pool: web::Data<Pool>,
    item: web::Json<InputGRN>,
) -> Result<GRN, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let new_grn = NewGRN {
        grn: &item.grn,
        challan_no: &item.challan_no,
        challan_date: chrono::NaiveDate::parse_from_str(&item.challan_date, "%d-%m-%Y").unwrap(),
        grade_name: &item.grade_name,
        size: &item.size,
        section: &item.section,
        heat_no: &item.heat_no,
        heat_code: match &item.heat_code.as_ref().unwrap().is_empty() {
            true => None,
            false => Some(&item.heat_code.as_ref().unwrap())
        },
        received_qty: &item.received_qty,
        created_by: &item.created_by
    };

    let res = insert_into(grns).values(&new_grn).get_result(&conn)?;
    Ok(res)
}

fn delete_single_grn(pool: web::Data<Pool>, gr_no: i64) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let count = delete(grns.find(gr_no)).execute(&conn)?;
    Ok(count)

}