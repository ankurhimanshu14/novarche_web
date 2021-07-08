-- Your SQL goes here

CREATE TABLE IF NOT EXISTS employees(
    id SERIAL,
    employee_id TEXT NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    first_name TEXT NOT NULL,
    middle_name TEXT,
    last_name TEXT NOT NULL,
    gender TEXT NOT NULL,
    dob TIMESTAMP NOT NULL,
    address TEXT,
    email TEXT,
    dept_id SERIAL NOT NULL,
    salary BIGSERIAL NOT NULL,
    doj TIMESTAMP NOT NULL,
    dol TIMESTAMP,
    created_on TIMESTAMP,
    created_by TEXT NOT NULL,
    modified_on TIMESTAMP,
    modified_by TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true
);