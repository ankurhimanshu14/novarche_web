table! {
    grades (grade_id) {
        grade_id -> Int4,
        grade_name -> Varchar,
        size -> Int4,
        section -> Varchar,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        email -> Varchar,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    grades,
    users,
);
