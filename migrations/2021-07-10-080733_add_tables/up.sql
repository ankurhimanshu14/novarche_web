CREATE TABLE IF NOT EXISTS persons(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    first_name TEXT NOT NULL,
    middle_name TEXT,
    last_name TEXT NOT NULL,
    gender TEXT NOT NULL,
    dob DATE NOT NULL,
    address TEXT,
    email TEXT,
    uidai BIGSERIAL NOT NULL,
    uan BIGSERIAL NOT NULL,
    pan TEXT,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by TEXT NOT NULL,
    modified_on TIMESTAMP,
    modified_by TEXT
);

CREATE TABLE IF NOT EXISTS employees(
    id SERIAL,
    person_id SERIAL,
    employee_id TEXT NOT NULL UNIQUE PRIMARY KEY,
    dept_id TEXT NOT NULL,
    salary SERIAL NOT NULL,
    doj DATE NOT NULL,
    dol DATE,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by TEXT NOT NULL,
    modified_on TIMESTAMP,
    modified_by TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_person
        FOREIGN KEY (person_id)
            REFERENCES persons(id)
            ON UPDATE CASCADE
            ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS users(
    id SERIAL,
    employee_id TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by TEXT NOT NULL,
    modified_on TIMESTAMP,
    modified_by TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,

    PRIMARY KEY(username),

    CONSTRAINT fk_user
        FOREIGN KEY (employee_id)
            REFERENCES employees(employee_id)
            ON UPDATE CASCADE
            ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS grades(
    id SERIAL,
    grade_name TEXT NOT NULL UNIQUE,
    size SERIAL NOT NULL,
    section TEXT NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by TEXT NOT NULL,
    modified_on TIMESTAMP,
    modified_by TEXT,

    PRIMARY KEY(grade_name)
);