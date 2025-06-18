-- Add migration script here
CREATE TABLE IF NOT EXISTS users(
    id SERIAL PRIMARY KEY,
    username VARCHAR(30) NOT NULL UNIQUE,
    email VARCHAR(40) UNIQUE,
    password_hash TEXT NOT NULL,
    is_active BOOLEAN,
    role VARCHAR(15) NOT NULL DEFAULT 'user'
);