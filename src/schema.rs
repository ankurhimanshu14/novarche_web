table! {
    approvals (rm_id) {
        id -> Int4,
        rm_id -> Int8,
        heat_no -> Text,
        part_no -> Int4,
        avail_qty -> Int4,
        created_on -> Timestamp,
        created_by -> Text,
        modified_on -> Nullable<Timestamp>,
        modified_by -> Nullable<Text>,
    }
}

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
        steel_code -> Text,
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
    grns (grn) {
        id -> Int4,
        grn -> Int8,
        challan_no -> Int8,
        challan_date -> Date,
        grade_name -> Text,
        size -> Int4,
        section -> Text,
        heat_no -> Text,
        heat_code -> Nullable<Text>,
        received_qty -> Int4,
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

joinable!(approvals -> grns (rm_id));
joinable!(employees -> persons (person_id));
joinable!(grns -> grades (grade_name));
joinable!(users -> employees (employee_id));

allow_tables_to_appear_in_same_query!(
    approvals,
    employees,
    grades,
    grns,
    persons,
    users,
);
