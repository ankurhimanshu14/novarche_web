#[path = "../schema.rs"]
mod schema;

use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize, Queryable, Eq, PartialEq)]
pub struct User {
    pub user_id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
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
    pub first_name: &'a str,
    pub middle_name: Option<&'a str>,
    pub last_name: &'a str,
    pub email: &'a str,
    pub username: &'a str,
    pub hash: &'a str,
}