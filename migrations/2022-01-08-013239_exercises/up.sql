-- Your SQL goes here

CREATE TABLE exercises (
  ex_id SERIAL PRIMARY KEY,
  u_id VARCHAR(50) NOT NULL,
  ex_name VARCHAR(255) NOT NULL,
  ex_description TEXT NOT NULL,
  ex_input TEXT NOT NULL,
  ex_answer TEXT NOT NULL,
  ex_created_at DATETIME NOT NULL DEFAULT NOW(),
  ex_updated_at DATETIME NOT NULL DEFAULT NOW()
);