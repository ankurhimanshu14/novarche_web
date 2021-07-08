#[path="../schema.rs"]
mod schema;

use serde::{ Deserialize, Serialize };
use crate::schema::*;

#[derive(Debug, Queryable, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct Employee {
    pub id: i32,
    pub employee_id: String,
    pub title: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub gender: String,
    pub dob: chrono::NaiveDateTime,
    pub address: Option<String>,
    pub email: Option<String>,
    pub dept_id: i32,
    pub salary: Option<i64>,
    pub doj: chrono::NaiveDateTime,
    pub dol: Option<chrono::NaiveDateTime>,
    pub created_on: Option<chrono::NaiveDateTime>,
    pub created_by: String,
    pub modified_on: Option<chrono::NaiveDateTime>,
    pub modified_by: Option<String>,
    pub is_active: bool
}

#[derive(Debug, Insertable, Identifiable)]
#[table_name = "employees"]
#[primary_key(employee_id)]
pub struct NewEmployee {
    pub employee_id: String,
    pub title: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub gender: String,
    pub dob: chrono::NaiveDateTime,
    pub address: Option<String>,
    pub email: Option<String>,
    pub dept_id: i32,
    pub salary: Option<i64>,
    pub doj: chrono::NaiveDateTime,
    pub dol: Option<chrono::NaiveDateTime>,
    pub created_on: Option<chrono::NaiveDateTime>,
    pub created_by: String,
    pub modified_on: Option<chrono::NaiveDateTime>,
    pub modified_by: Option<String>,
    pub is_active: bool
}