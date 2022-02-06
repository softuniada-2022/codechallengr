-- Your SQL goes here

CREATE TABLE likes (
    u_id VARCHAR(50),
    ex_id INTEGER,
    PRIMARY KEY (u_id, ex_id)
);