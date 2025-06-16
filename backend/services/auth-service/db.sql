CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    username VARCHAR(30) NOT NULL,
    email VARCHAR(40) UNIQUE,
    hashed_password TEXT NOT NULL,
    role VARCHAR(15) NOT NULL DEFAULT 'user'
);


INSERT INTO users (username, hashed_password) VALUES('#', '#');

INSERT INTO users (username, hashed_password, email) VALUES('#', '#', 'example@com');

INSERT INTO users (username, hashed_password, role, email) VALUES('#', '#', 'pidor', 'example@com');

SELECT * FROM users;
SELECT role FROM users WHERE username = '#';
SELECT role, hashed_password FROM users WHERE username ='#' OR email = '#';

UPDATE users SET role = 'pidor'  WHERE username ='#' AND hashed_password = '#';

DELETE FROM users WHERE username ='#' AND hashed_password = '#';
