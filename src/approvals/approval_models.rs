#[path="../schema.rs"]
mod schema;

use serde::{ Deserialize, Serialize };
use crate::schema::*;

#[derive(Debug, Queryable, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct Approval {
    pub id: i32,
    pub rm_id: i64,
    pub heat_no: String,
    pub part_no: i32,
    pub avail_qty: i32,
    pub created_on: chrono::NaiveDateTime,
    pub created_by: String,
    pub modified_on: Option<chrono::NaiveDateTime>,
    pub modified_by: Option<String>
}

#[derive(Debug, Insertable, Identifiable)]
#[table_name = "approvals"]
#[primary_key(rm_id)]
pub struct NewApproval<'a> {
    pub rm_id: &'a i64,
    pub heat_no: &'a str,
    pub part_no: &'a i32,
    pub created_by: &'a str
}