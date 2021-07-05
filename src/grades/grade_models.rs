#[path = "../schema.rs"]
mod schema;

use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize, Queryable)]
pub struct Grade {
    pub grade_id: i32,
    pub grade_name: String,
    pub size: i32,
    pub section: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[table_name = "grades"]
pub struct NewGrade<'a> {
    pub grade_name: &'a str,
    pub size: &'a i32,
    pub section: &'a str,
}