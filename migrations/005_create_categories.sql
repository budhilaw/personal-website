-- 005: Create categories table
-- Migration: Blog post categories

CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index for slug lookups
CREATE INDEX idx_categories_slug ON categories(slug);

-- Seed default categories
INSERT INTO categories (name, slug, description) VALUES
    ('Technology', 'technology', 'Tech-related posts'),
    ('Programming', 'programming', 'Programming tutorials and tips'),
    ('Life', 'life', 'Personal life stories');
