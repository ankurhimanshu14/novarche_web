#[path = "../schema.rs"]
mod schema;

use crate::schema::*;
use serde::{Deserialize, Serialize};
use std::io::Write;
use diesel::backend::Backend;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, AsExpression, Eq, PartialEq)]
pub enum Role {
    Anonymous,
    Authenticated,
    Administrator
}

impl<Db: Backend> ToSql<User, Db> for Role {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Db>) -> serialize::Result {
        match *self {
            Role::Anonymous => out.write_all(b"anonymous")?,
            Role::Authenticated => out.write_all(b"authenticated")?,
            Role::Administrator => out.write_all(b"administrator")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<User, Pg> for Role {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"anonymous" => Ok(Role::Anonymous),
            b"authenticated" => Ok(Role::Authenticated),
            b"administrator" => Ok(Role::Administrator),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, AsExpression, Eq, PartialEq)]
pub enum Userstatus {
    Active,
    Inactive,
    Suspended
}

impl<Db: Backend> ToSql<User, Db> for Userstatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Db>) -> serialize::Result {
        match *self {
            Userstatus::Active => out.write_all(b"active")?,
            Userstatus::Inactive => out.write_all(b"inactive")?,
            Userstatus::Suspended => out.write_all(b"suspended")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<User, Pg> for Userstatus {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"active" => Ok(Userstatus::Active),
            b"inactive" => Ok(Userstatus::Inactive),
            b"suspended" => Ok(Userstatus::Suspended),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug,Serialize, Deserialize, Queryable, Eq, PartialEq)]
pub struct User {
    pub user_id: i32,
    pub employee_id: String,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub hash: String,
    pub role: Role,
    pub status: Userstatus,
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
    pub role: &'a Role
}