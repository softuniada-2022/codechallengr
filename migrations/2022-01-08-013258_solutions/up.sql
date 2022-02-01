-- Your SQL goes here

CREATE TABLE solutions (
  s_id SERIAL PRIMARY KEY,
  ex_id INTEGER NOT NULL,
  u_id TEXT NOT NULL,
  s_answer TEXT NOT NULL,
  s_correct BOOLEAN NOT NULL,
  s_submitted_at DATETIME NOT NULL DEFAULT NOW()
);