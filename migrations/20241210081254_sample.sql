-- Add migration script here

CREATE TABLE IF NOT EXISTS Sample (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);