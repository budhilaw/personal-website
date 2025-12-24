-- 002: Create permissions table
-- Migration: Permissions for fine-grained access control

CREATE TABLE permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) UNIQUE NOT NULL,       -- e.g., 'posts:read', 'posts:create'
    description TEXT,
    resource VARCHAR(50) NOT NULL,           -- e.g., 'posts', 'categories', 'tags'
    action VARCHAR(50) NOT NULL,             -- e.g., 'read', 'create', 'update', 'delete', 'publish'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(resource, action)
);

-- Seed default permissions
INSERT INTO permissions (name, description, resource, action) VALUES
    -- Posts permissions
    ('posts:read', 'View posts', 'posts', 'read'),
    ('posts:create', 'Create new posts', 'posts', 'create'),
    ('posts:update', 'Update own posts', 'posts', 'update'),
    ('posts:delete', 'Delete posts', 'posts', 'delete'),
    ('posts:publish', 'Publish/unpublish posts', 'posts', 'publish'),
    
    -- Categories permissions
    ('categories:read', 'View categories', 'categories', 'read'),
    ('categories:create', 'Create categories', 'categories', 'create'),
    ('categories:update', 'Update categories', 'categories', 'update'),
    ('categories:delete', 'Delete categories', 'categories', 'delete'),
    
    -- Tags permissions
    ('tags:read', 'View tags', 'tags', 'read'),
    ('tags:create', 'Create tags', 'tags', 'create'),
    ('tags:update', 'Update tags', 'tags', 'update'),
    ('tags:delete', 'Delete tags', 'tags', 'delete'),
    
    -- Users permissions (admin only)
    ('users:read', 'View users', 'users', 'read'),
    ('users:create', 'Create users', 'users', 'create'),
    ('users:update', 'Update users', 'users', 'update'),
    ('users:delete', 'Delete users', 'users', 'delete');
