#[path = "../schema.rs"]
mod schema;

#[path = "../utils.rs"]
mod utils;

use super::approval_models::{NewApproval, Approval};
use crate::schema::approvals::dsl::*;
use crate::utils::Pool;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputApproval {
    pub rm_id: i64,
    pub heat_no: String,
    pub part_no: i32,
    pub created_by: String,
}

//Handler for GET /approvals
pub async fn get_approvals(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_approvals(db))
    .await
    .map(|approval| HttpResponse::Ok().json(approval))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for GET /approvals/{id}
pub async fn get_approval_by_id(
    db: web::Data<Pool>,
    r_id: web::Path<i64>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_approval_by_id(db, r_id.into_inner()))
            .await
            .map(|gate_entry| HttpResponse::Ok().json(gate_entry))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for POST /approvals
pub async fn add_approval(
    db: web::Data<Pool>,
    item: web::Json<InputApproval>,
) -> Result<HttpResponse, Error> {    
    Ok(web::block(move || add_single_approval(db, item))
        .await
        .map(|approval| HttpResponse::Created().json(approval))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for DELETE /approvals/{id}
pub async fn delete_approval(
    db: web::Data<Pool>,
    r_id: web::Path<i64>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_approval(db, r_id.into_inner()))
            .await
            .map(|approval| HttpResponse::Ok().json(approval))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn get_all_approvals(pool: web::Data<Pool>) -> Result<Vec<Approval>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = approvals.load::<Approval>(&conn)?;
    Ok(items)
}

fn db_get_approval_by_id(pool: web::Data<Pool>, r_id: i64) -> Result<Approval, diesel::result::Error> {
    let conn = pool.get().unwrap();
    approvals.find(r_id).get_result::<Approval>(&conn)
}

fn add_single_approval(
    pool: web::Data<Pool>,
    item: web::Json<InputApproval>,
) -> Result<Approval, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let new_approval = NewApproval {
        rm_id: &item.rm_id,
        heat_no: &item.heat_no,
        part_no: &item.part_no,
        created_by: &item.created_by,
    };

    let res = insert_into(approvals).values(&new_approval).get_result(&conn)?;
    Ok(res)
}

fn delete_single_approval(pool: web::Data<Pool>, r_id: i64) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let count = delete(approvals.find(r_id)).execute(&conn)?;
    Ok(count)
}