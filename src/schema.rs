table! {
    users (id) {
        user_id -> Int4,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        email -> Varchar,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>
    }
}