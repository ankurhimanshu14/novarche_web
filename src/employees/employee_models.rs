#[path = "../schema.rs"]
mod schema;

use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Eq, PartialEq)]
pub struct Employee {
    pub emp_id: i32,
    pub employee_id: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub dob: chrono::NaiveDateTime,
    pub doj: chrono::NaiveDateTime,
    pub dol: Option<chrono::NaiveDateTime>,
    pub uidai: i32,
    pub pan: String,
    pub uan: i32,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[table_name = "employees"]
pub struct NewEmployee<'a> {
    pub employee_id: &'a str,
    pub first_name: &'a str,
    pub middle_name: Option<&'a str>,
    pub last_name: &'a str,
    pub uidai: &'a i32,
    pub pan: &'a str,
    pub uan: &'a i32
}