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
    pub salary: i32,
    pub doj: chrono::NaiveDateTime,
    pub dol: Option<chrono::NaiveDateTime>,
    pub uidai: i32,
    pub uan: i32,
    pub pan: Option<String>,
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
    pub employee_id: &'a str,
    pub title: &'a str,
    pub first_name: &'a str,
    pub middle_name: Option<&'a str>,
    pub last_name: &'a str,
    pub gender: &'a str,
    pub dob: chrono::NaiveDateTime,
    pub address: Option<&'a str>,
    pub email: Option<&'a str>,
    pub dept_id: &'a i32,
    pub salary: &'a i32,
    pub doj: chrono::NaiveDateTime,
    pub uidai: &'a i32,
    pub uan: &'a i32,
    pub pan: Option<&'a str>,
    pub created_by: &'a str
}