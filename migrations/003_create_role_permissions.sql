-- 003: Create role_permissions junction table
-- Migration: Maps roles to their permissions

CREATE TABLE role_permissions (
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (role_id, permission_id)
);

-- Create index for faster permission lookups
CREATE INDEX idx_role_permissions_role ON role_permissions(role_id);

-- Seed role permissions
-- Get role IDs and assign permissions

-- Admin: all permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id 
FROM roles r, permissions p 
WHERE r.slug = 'admin';

-- Editor: content management (all post/category/tag permissions)
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id 
FROM roles r, permissions p 
WHERE r.slug = 'editor' 
  AND p.name IN (
    'posts:read', 'posts:create', 'posts:update', 'posts:delete', 'posts:publish',
    'categories:read', 'categories:create', 'categories:update', 'categories:delete',
    'tags:read', 'tags:create', 'tags:update', 'tags:delete'
  );

-- Writer: create/edit own posts
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id 
FROM roles r, permissions p 
WHERE r.slug = 'writer' 
  AND p.name IN ('posts:read', 'posts:create', 'posts:update', 'categories:read', 'tags:read');

-- Viewer: read-only
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id 
FROM roles r, permissions p 
WHERE r.slug = 'viewer' 
  AND p.name IN ('posts:read', 'categories:read', 'tags:read');
