-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users
(
    id            UUID PRIMARY KEY     DEFAULT uuid_generate_v4(),
    email         TEXT UNIQUE NOT NULL,
    password      TEXT        NOT NULL,
    login_session VARCHAR     NOT NULL,
    created_at    TIMESTAMP   NOT NULL DEFAULT current_timestamp,
    updated_at    TIMESTAMP
);
