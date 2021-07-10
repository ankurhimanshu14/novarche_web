#[path="../schema.rs"]
mod schema;

use serde::{ Deserialize, Serialize };
use crate::schema::*;

#[derive(Debug, Queryable, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct Grade {
    pub id: i32,
    pub grade_name: String,
    pub size: i32,
    pub section: String,
    pub created_on: chrono::NaiveDateTime,
    pub created_by: String,
    pub modified_on: Option<chrono::NaiveDateTime>,
    pub modified_by: Option<String>
}

#[derive(Debug, Insertable, Identifiable)]
#[table_name = "grades"]
#[primary_key(grade_name)]
pub struct NewGrade<'a> {
    pub grade_name: &'a str,
    pub size: &'a i32,
    pub section: &'a str,
    pub created_by: &'a str
}