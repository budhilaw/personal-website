-- 001: Create roles table
-- Migration: Dynamic roles with CRUD support

CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ  -- Soft delete
);

-- Create index for slug lookups
CREATE INDEX idx_roles_slug ON roles(slug);
CREATE INDEX idx_roles_deleted_at ON roles(deleted_at);

-- Seed default roles
INSERT INTO roles (name, slug, description) VALUES
    ('Administrator', 'admin', 'Full system access'),
    ('Editor', 'editor', 'Can publish and manage all content'),
    ('Writer', 'writer', 'Can create and edit own content'),
    ('Viewer', 'viewer', 'Read-only access');
