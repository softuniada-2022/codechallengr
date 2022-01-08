-- Your SQL goes here

CREATE TABLE exercises (
  ex_id SERIAL PRIMARY KEY,
  t_name VARCHAR(255) NOT NULL,
  t_description TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);