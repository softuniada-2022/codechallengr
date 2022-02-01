-- Your SQL goes here


CREATE TABLE users (
    u_name VARCHAR(50) PRIMARY KEY,
    u_email VARCHAR(255) NOT NULL,
    u_password VARCHAR(512) NOT NULL,
    u_permission INTEGER NOT NULL DEFAULT 0,
    u_created_at DATETIME NOT NULL DEFAULT NOW(),
    u_updated_at DATETIME NOT NULL DEFAULT NOW()
);
