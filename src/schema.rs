table! {
    employees (employee_id) {
        id -> Int4,
        employee_id -> Text,
        title -> Text,
        first_name -> Text,
        middle_name -> Nullable<Text>,
        last_name -> Text,
        gender -> Text,
        dob -> Timestamp,
        address -> Nullable<Text>,
        email -> Nullable<Text>,
        dept_id -> Int4,
        salary -> Int8,
        doj -> Timestamp,
        dol -> Nullable<Timestamp>,
        created_on -> Nullable<Timestamp>,
        created_by -> Text,
        modified_on -> Nullable<Timestamp>,
        modified_by -> Nullable<Text>,
        is_active -> Bool,
    }
}
