CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    username VARCHAR(30) NOT NULL UNIQUE,
    email VARCHAR(40) UNIQUE,
    password_hash TEXT NOT NULL,
    is_active BOOLEAN,
    role VARCHAR(15) NOT NULL DEFAULT 'user'
);


INSERT INTO users (username, password_hash) VALUES('#', '#');

INSERT INTO users (username, password_hash, email) VALUES('#', '#', 'example@com');

INSERT INTO users (username, password_hash, role, email) VALUES('#', '#', 'pidor', 'example@com');

SELECT * FROM users;
SELECT role FROM users WHERE username = '#';
SELECT role, password_hash FROM users WHERE username ='#' OR email = '#';
SELECT username, role, password_hash FROM users WHERE username= $1;

UPDATE users SET role = 'pidor'  WHERE username ='#' AND password_hash = '#';

DELETE FROM users WHERE username ='#' AND password_hash = '#';
