-- Your SQL goes here

CREATE TABLE solutions (
  s_id SERIAL PRIMARY KEY,
  ex_id INTEGER NOT NULL,
  u_id INTEGER NOT NULL,
  s_submitted_at DATETIME NOT NULL DEFAULT NOW()
);