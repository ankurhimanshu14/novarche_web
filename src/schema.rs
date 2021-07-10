table! {
    employees (employee_id) {
        id -> Int4,
        person_id -> Int4,
        employee_id -> Text,
        dept_id -> Text,
        salary -> Int4,
        doj -> Date,
        dol -> Nullable<Date>,
        created_on -> Timestamp,
        created_by -> Text,
        modified_on -> Nullable<Timestamp>,
        modified_by -> Nullable<Text>,
        is_active -> Bool,
    }
}

table! {
    grades (grade_name) {
        id -> Int4,
        grade_name -> Text,
        size -> Int4,
        section -> Text,
        created_on -> Timestamp,
        created_by -> Text,
        modified_on -> Nullable<Timestamp>,
        modified_by -> Nullable<Text>,
    }
}

table! {
    persons (id) {
        id -> Int4,
        title -> Text,
        first_name -> Text,
        middle_name -> Nullable<Text>,
        last_name -> Text,
        gender -> Text,
        dob -> Date,
        address -> Nullable<Text>,
        email -> Nullable<Text>,
        uidai -> Int8,
        uan -> Int8,
        pan -> Nullable<Text>,
        created_on -> Timestamp,
        created_by -> Text,
        modified_on -> Nullable<Timestamp>,
        modified_by -> Nullable<Text>,
    }
}

table! {
    users (username) {
        id -> Int4,
        employee_id -> Text,
        username -> Text,
        password -> Text,
        created_on -> Timestamp,
        created_by -> Text,
        modified_on -> Nullable<Timestamp>,
        modified_by -> Nullable<Text>,
        is_active -> Bool,
    }
}

joinable!(employees -> persons (person_id));
joinable!(users -> employees (employee_id));

allow_tables_to_appear_in_same_query!(
    employees,
    grades,
    persons,
    users,
);
