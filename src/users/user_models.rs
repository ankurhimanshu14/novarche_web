#[path = "../schema.rs"]
mod schema;

use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize, Queryable, Eq, PartialEq)]
pub struct User {
    pub user_id: i32,
    pub employee_id: String,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub hash: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub employee_id: &'a str,
    pub email: &'a str,
    pub username: &'a str,
    pub hash: &'a str,
}