table! {
    use diesel::sql_types::*;
    use crate::employees::employee_models::Title;

    employees (employee_id) {
        id -> Int4,
        employee_id -> Varchar,
        title -> Title,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        dob -> Timestamp,
        doj -> Timestamp,
        dol -> Nullable<Timestamp>,
        uidai -> Int8,
        uan -> Int8,
        pan -> Text,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;

    grades (grade_name) {
        id -> Int4,
        grade_name -> Varchar,
        size -> Int4,
        section -> Varchar,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::users::user_models::{Role, Userstatus};

    users (username) {
        id -> Int4,
        employee_id -> Varchar,
        email -> Varchar,
        username -> Varchar,
        hash -> Varchar,
        role -> Role,
        status -> Userstatus,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
    }
}

joinable!(users -> employees (employee_id));

allow_tables_to_appear_in_same_query!(
    employees,
    grades,
    users,
);
