-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    skill_level INT NOT NULL,
    preferred_climbing_style VARCHAR,
    preferred_gym VARCHAR
);
