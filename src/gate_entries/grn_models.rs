#[path="../schema.rs"]
mod schema;

use serde::{ Deserialize, Serialize };
use crate::schema::*;

#[derive(Debug, Queryable, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct GRN {
    pub id: i32,
    pub grn: i64,
    pub challan_no: i64,
    pub challan_date: chrono::NaiveDate,
    pub grade_name: String,
    pub size: i32,
    pub section: String,
    pub heat_no: String,
    pub heat_code: Option<String>,
    pub received_qty: i32,
    pub created_on: chrono::NaiveDateTime,
    pub created_by: String,
    pub modified_on: Option<chrono::NaiveDateTime>,
    pub modified_by: Option<String>
}

#[derive(Debug, Insertable, Identifiable)]
#[table_name = "grns"]
#[primary_key(grn)]
pub struct NewGRN<'a> {
    pub grn: &'a i64,
    pub challan_no: &'a i64,
    pub challan_date: chrono::NaiveDate,
    pub grade_name: &'a String,
    pub size: &'a i32,
    pub section: &'a String,
    pub heat_no: &'a String,
    pub heat_code: Option<&'a str>,
    pub received_qty: &'a i32,
    pub created_by: &'a str
}