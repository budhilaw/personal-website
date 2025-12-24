-- 006: Create tags table
-- Migration: Blog post tags

CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL,
    slug VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index for slug lookups
CREATE INDEX idx_tags_slug ON tags(slug);

-- Seed default tags
INSERT INTO tags (name, slug) VALUES
    ('Rust', 'rust'),
    ('Go', 'go'),
    ('TypeScript', 'typescript'),
    ('React', 'react'),
    ('Tutorial', 'tutorial');
