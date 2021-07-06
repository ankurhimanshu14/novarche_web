-- Your SQL goes here

CREATE TABLE IF NOT EXISTS employees(
    id SERIAL PRIMARY KEY,
    employee_id VARCHAR NOT NULL UNIQUE,
    first_name VARCHAR NOT NULL,
    middle_name VARCHAR,
    last_name VARCHAR NOT NULL,
    uidai SERIAL NOT NULL,
    uan SERIAL NOT NULL,
    pan TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS users(
    id SERIAL PRIMARY KEY,
    employee_id VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL,
    username VARCHAR NOT NULL UNIQUE,
    hash VARCHAR NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'ACTIVE',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_at TIMESTAMP,
    CONSTRAINT fk_employees
        FOREIGN KEY(employee_id)
            REFERENCES employees(employee_id)
            ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS grades(
    id SERIAL PRIMARY KEY,
    grade_name VARCHAR NOT NULL,
    size INT NOT NULL,
    section VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_at TIMESTAMP
);