table! {
    employees (id) {
        id -> Int4,
        employee_id -> Varchar,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        uidai -> Int4,
        uan -> Int4,
        pan -> Text,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
    }
}

table! {
    grades (id) {
        id -> Int4,
        grade_name -> Varchar,
        size -> Int4,
        section -> Varchar,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        employee_id -> Varchar,
        email -> Varchar,
        username -> Varchar,
        hash -> Varchar,
        status -> Varchar,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    employees,
    grades,
    users,
);
