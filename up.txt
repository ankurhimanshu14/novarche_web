-- Your SQL goes here

CREATE TYPE Title AS ENUM (
    'Mr.', 'Mrs.', 'Ms.'
);

CREATE TYPE Role AS ENUM (
    'Anonymous', 'Authenticated', 'Administrator'
);

CREATE TYPE USerStatus AS ENUM (
    'Active', 'Suspended', 'Inactive'
);

CREATE TABLE IF NOT EXISTS employees(
    id SERIAL,
    employee_id VARCHAR NOT NULL UNIQUE,
    title Title NOT NULL,
    first_name VARCHAR NOT NULL,
    middle_name VARCHAR,
    last_name VARCHAR NOT NULL,
    dob TIMESTAMP NOT NULL,
    doj TIMESTAMP NOT NULL,
    dol TIMESTAMP,
    uidai BIGSERIAL NOT NULL,
    uan BIGSERIAL NOT NULL,
    pan TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_at TIMESTAMP,

    PRIMARY KEY (employee_id)
);

CREATE TABLE IF NOT EXISTS users(
    id SERIAL,
    employee_id VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL,
    username VARCHAR NOT NULL UNIQUE,
    hash VARCHAR NOT NULL,
    role Role NOT NULL,
    status UserStatus NOT NULL DEFAULT 'Active',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_at TIMESTAMP,

    PRIMARY KEY (username),
    
    CONSTRAINT fk_employees
        FOREIGN KEY(employee_id)
            REFERENCES employees(employee_id)
            ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS grades(
    id SERIAL,
    grade_name VARCHAR NOT NULL UNIQUE,
    size INT NOT NULL,
    section VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_at TIMESTAMP,

    PRIMARY KEY (grade_name)
);