#[path="../schema.rs"]
mod schema;

use serde::{ Deserialize, Serialize };
use crate::schema::*;

#[derive(Debug, Queryable, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct Employee {
    pub id: i32,
    pub person_id: i32,
    pub employee_id: String,
    pub dept_id: String,
    pub salary: i32,
    pub doj: chrono::NaiveDate,
    pub dol: Option<chrono::NaiveDate>,
    pub created_on: chrono::NaiveDateTime,
    pub created_by: String,
    pub modified_on: Option<chrono::NaiveDateTime>,
    pub modified_by: Option<String>,
    pub is_active: bool
}

#[derive(Debug, Insertable, Identifiable)]
#[table_name = "employees"]
#[primary_key(employee_id)]
pub struct NewEmployee<'a> {
    pub person_id: &'a i32,
    pub employee_id: &'a str,
    pub dept_id: &'a str,
    pub salary: &'a i32,
    pub doj: chrono::NaiveDate,
    pub created_by: &'a str
}