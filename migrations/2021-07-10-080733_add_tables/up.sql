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
    steel_code TEXT NOT NULL UNIQUE,
    grade_name TEXT NOT NULL UNIQUE,
    size SERIAL NOT NULL,
    section TEXT NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by TEXT NOT NULL,
    modified_on TIMESTAMP,
    modified_by TEXT,

    PRIMARY KEY(grade_name)
);

CREATE TABLE IF NOT EXISTS GRNs(
    id SERIAL,
    grn BIGINT NOT NULL UNIQUE,
    challan_no BIGINT NOT NULL,
    challan_date DATE NOT NULL,
    grade_name TEXT NOT NULL,
    size SERIAL NOT NULL,
    section TEXT NOT NULL,
    heat_no TEXT NOT NULL,
    heat_code TEXT,
    received_qty SERIAL NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by TEXT NOT NULL,
    modified_on TIMESTAMP,
    modified_by TEXT,

    PRIMARY KEY(grn),

    CONSTRAINT fk_grade
    FOREIGN KEY (grade_name)
        REFERENCES grades(grade_name)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE TEMPORARY TABLE temp_approvals(
    approval_id SERIAL PRIMARY KEY,
    heat_no TEXT NOT NULL,
    part_no SERIAL NOT NULL
);

CREATE TABLE IF NOT EXISTS approvals(
    id SERIAL,
    rm_id BIGSERIAL NOT NULL,
    heat_no TEXT NOT NULL,
    part_no SERIAL NOT NULL,
    avail_qty SERIAL NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by TEXT NOT NULL,
    modified_on TIMESTAMP,
    modified_by TEXT,
    PRIMARY KEY (rm_id),
    CONSTRAINT fk_aprovals
        FOREIGN KEY(rm_id)
            REFERENCES grns(grn)
            ON UPDATE CASCADE
            ON DELETE CASCADE
);

CREATE FUNCTION insert_approvals() RETURNS TRIGGER AS
    $BODY$
    BEGIN
        INSERT INTO approvals (rm_id, heat_no, part_no, avail_qty)
        SELECT
        g.grn,
        NEW.heat_no,
        NEW.part_no,
        g.received_qty
        FROM grns g
        WHERE g.heat_no = NEW.heat_no;
    END
    $BODY$
LANGUAGE 'plpgsql';

CREATE TRIGGER after_approved_components_insert
    AFTER INSERT
    ON temp_approvals
    FOR EACH ROW
EXECUTE PROCEDURE insert_approvals();