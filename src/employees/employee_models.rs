#[path = "../schema.rs"]
mod schema;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, AsExpression, Eq, PartialEq)]
pub enum Title {
    Mr,
    Mrs,
    Ms
}

use crate::schema::*;
use serde::{Deserialize, Serialize};
use std::io::Write;
use diesel::backend::Backend;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;

#[derive(Debug,Serialize, Deserialize, Queryable)]
pub struct Employee {
    pub emp_id: i32,
    pub employee_id: String,
    pub title: Title,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub uidai: i64,
    pub pan: String,
    pub uan: i64,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[table_name = "employees"]
pub struct NewEmployee<'a> {
    pub employee_id: &'a str,
    pub title: &'a Title,
    pub first_name: &'a str,
    pub middle_name: Option<&'a str>,
    pub last_name: &'a str,
    pub uidai: &'a i64,
    pub pan: &'a str,
    pub uan: &'a i64
}

impl<Db: Backend> ToSql<Employee, Db> for Title {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Db>) -> serialize::Result {
        match *self {
            Title::Mr => out.write_all(b"Mr.")?,
            Title::Mrs => out.write_all(b"Mrs.")?,
            Title::Ms => out.write_all(b"Ms.")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Employee, Pg> for Title {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"Mr." => Ok(Title::Mr),
            b"Mrs." => Ok(Title::Mrs),
            b"Ms." => Ok(Title::Ms),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl<Db: Backend> ToSql<Employee, Db> for chrono::NaiveDate {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Db>) -> serialize::Result {
        out.write_all(b"%d-%m-%Y")?;

        Ok(IsNull::No)
    }
}

impl FromSql<Employee, Pg> for chrono::NaiveDate {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<chrono::NaiveDate> {
        chrono::NaiveDate::parse_from_str(bytes, "%d-%m-%Y")
    }
}