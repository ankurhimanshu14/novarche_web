#[path="../schema.rs"]
mod schema;

use serde::{ Deserialize, Serialize };
use crate::schema::*;

#[derive(Debug, Queryable, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct User {
    pub id: i32,
    pub employee_id: String,
    pub username: String,
    password: String,
    pub created_on: chrono::NaiveDateTime,
    pub created_by: String,
    pub modified_on: Option<chrono::NaiveDateTime>,
    pub modified_by: Option<String>,
    pub is_active: bool
}

#[derive(Debug, Insertable, Identifiable)]
#[table_name = "users"]
#[primary_key(username)]
pub struct NewUser<'a> {
    pub employee_id: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub created_by: &'a str
}