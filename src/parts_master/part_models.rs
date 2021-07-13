#[path="../schema.rs"]
mod schema;

use serde::{ Deserialize, Serialize };
use crate::schema::*;

#[derive(Debug, Queryable, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Part {
    pub id: i32,
    pub part_code: String,
    pub part_no: i32,
    pub part_name: String,
    pub material: String,
    pub forging_wt: f32,
    pub cut_wt: f32,
    pub created_on: chrono::NaiveDateTime,
    pub created_by: String,
    pub modified_on: Option<chrono::NaiveDateTime>,
    pub modified_by: Option<String>
}

#[derive(Debug, Insertable, Identifiable)]
#[table_name = "parts"]
#[primary_key(part_no)]
pub struct NewPart<'a> {
    pub part_code: &'a str,
    pub part_no: &'a i32,
    pub part_name: &'a str,
    pub material: &'a str,
    pub forging_wt: &'a f32,
    pub cut_wt: &'a f32,
    pub created_by: &'a str
}