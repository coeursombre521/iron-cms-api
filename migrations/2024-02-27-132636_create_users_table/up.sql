-- Your SQL goes here

CREATE TABLE users (
    id                  UUID PRIMARY KEY,
    role                INTEGER NOT NULL,
    name                VARCHAR(255) NOT NULL,
    email               VARCHAR(255) NOT NULL UNIQUE,
    password_hash       VARCHAR(255) NOT NULL,
    reset_token         VARCHAR(255),
    reset_token_expiry  TIMESTAMP WITH TIME ZONE,
    created_at          TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at          TIMESTAMP WITH TIME ZONE
);
