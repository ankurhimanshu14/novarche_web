#[path="../schema.rs"]
mod schema;

use serde::{ Deserialize, Serialize };
use crate::schema::*;

#[derive(Debug, Queryable, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct Person {
    pub id: i32,
    pub title: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub gender: String,
    pub dob: chrono::NaiveDate,
    pub address: Option<String>,
    pub email: Option<String>,
    pub uidai: i64,
    pub uan: i64,
    pub pan: Option<String>,
    pub created_on: chrono::NaiveDateTime,
    pub created_by: String,
    pub modified_on: Option<chrono::NaiveDateTime>,
    pub modified_by: Option<String>
}

#[derive(Debug, Insertable)]
#[table_name = "persons"]
pub struct NewPerson<'a> {
    pub title: &'a str,
    pub first_name: &'a str,
    pub middle_name: Option<&'a str>,
    pub last_name: &'a str,
    pub gender: &'a str,
    pub dob: chrono::NaiveDate,
    pub address: Option<&'a str>,
    pub email: Option<&'a str>,
    pub uidai: &'a i64,
    pub uan: &'a i64,
    pub pan: Option<&'a str>,
    pub created_by: &'a str
}