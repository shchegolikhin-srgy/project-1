CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    external_id UUID DEFAULT uuid_generate_v4() UNIQUE NOT NULL,
    username VARCHAR(30) NOT NULL UNIQUE,
    email VARCHAR(40) UNIQUE,
    password_hash TEXT NOT NULL,
    is_active BOOLEAN,
    role VARCHAR(15) NOT NULL DEFAULT 'user'
);
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

INSERT INTO users (username, password_hash) VALUES('#', '#');

INSERT INTO users (username, password_hash, email) VALUES('#', '#', 'example@com');

SELECT * FROM users;
SELECT role FROM users WHERE username = '#';
SELECT role, password_hash FROM users WHERE username ='#' OR email = '#';
SELECT username, role, password_hash FROM users WHERE username= $1;

UPDATE users SET role = 'pidor'  WHERE username ='#' AND password_hash = '#';

DELETE FROM users WHERE username ='#';

CREATE TABLE refresh_tokens(
    id SERIAL PRIMARY KEY,
    token TEXT NOT NULL,
    user_id INTEGER REFERENCES users(id)
);

DELETE FROM refresh_tokens
USING users
WHERE refresh_tokens.user_id = users.id
AND users.username = $1
AND refresh_tokens.token = $2;

INSERT INTO refresh_tokens(token, user_id) 
SELECT $1, users.id 
FROM users 
WHERE users.username = $2;

SELECT refresh_tokens.token 
FROM refresh_tokens 
JOIN users ON refresh_tokens.user_id = users.id 
WHERE users.username = $1;