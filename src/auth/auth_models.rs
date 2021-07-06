#[path = "../schema.rs"]
mod schema;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String
}