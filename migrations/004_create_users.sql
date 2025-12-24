-- 004: Create users table
-- Migration: Users with role foreign key

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    role_id UUID NOT NULL REFERENCES roles(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ  -- Soft delete
);

-- Create indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role_id);
CREATE INDEX idx_users_deleted_at ON users(deleted_at);

-- Seed default users (password: admin123)
-- Hash generated with Argon2
INSERT INTO users (email, password_hash, name, role_id)
SELECT 'admin@example.com', '$argon2id$v=19$m=19456,t=2,p=1$ZG1oYXNobmFzaGRhc2Q$qqsv5TAEM9fvR+10stvbgg', 'Admin User', id
FROM roles WHERE slug = 'admin';

INSERT INTO users (email, password_hash, name, role_id)
SELECT 'editor@example.com', '$argon2id$v=19$m=19456,t=2,p=1$ZG1oYXNobmFzaGRhc2Q$qqsv5TAEM9fvR+10stvbgg', 'Editor User', id
FROM roles WHERE slug = 'editor';

INSERT INTO users (email, password_hash, name, role_id)
SELECT 'writer@example.com', '$argon2id$v=19$m=19456,t=2,p=1$ZG1oYXNobmFzaGRhc2Q$qqsv5TAEM9fvR+10stvbgg', 'Writer User', id
FROM roles WHERE slug = 'writer';

INSERT INTO users (email, password_hash, name, role_id)
SELECT 'viewer@example.com', '$argon2id$v=19$m=19456,t=2,p=1$ZG1oYXNobmFzaGRhc2Q$qqsv5TAEM9fvR+10stvbgg', 'Viewer User', id
FROM roles WHERE slug = 'viewer';
