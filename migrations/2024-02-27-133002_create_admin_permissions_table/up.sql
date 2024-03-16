-- Your SQL goes here

CREATE TABLE admin_permissions (
    id              UUID PRIMARY KEY,
    user_id         UUID REFERENCES users(id) NOT NULL,
    permission      INTEGER NOT NULL,
    created_at      TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at      TIMESTAMP WITH TIME ZONE
);
