-- Your SQL goes here

CREATE TABLE users (
    u_id SERIAL PRIMARY KEY,
    u_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    u_password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
