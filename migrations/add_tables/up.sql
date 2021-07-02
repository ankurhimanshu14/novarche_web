CREATE TABLE IF NOT EXISTS users(
    user_id SERIAL PRIMARY KEY
    first_name VARCHAR NOT NULL,
    middle_name VARCHAR,
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    username VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    created_at DATETIME NOT NULL DEFAULT NOW(),
    modified_at DATETIME
);