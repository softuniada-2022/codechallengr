-- Your SQL goes here


CREATE TABLE users (
    u_name VARCHAR(50) PRIMARY KEY,
    u_email VARCHAR(255) NOT NULL,
    u_password VARCHAR(512) NOT NULL,
    u_permission INTEGER NOT NULL DEFAULT 0,
    u_score INTEGER NOT NULL DEFAULT 0,
    u_created_at DATETIME NOT NULL DEFAULT NOW(),
    u_updated_at DATETIME NOT NULL DEFAULT NOW()
);

INSERT INTO users SET u_name = 'admin', u_email = 'admin@admin.com', u_password = '$2a$12$bj11FO/8kIVpxmIou1WWfui9yRiFPw/6maefnycIffLPl/42wj5fe', u_permission = 3, u_score = 0, u_created_at = NOW(), u_updated_at = NOW();
