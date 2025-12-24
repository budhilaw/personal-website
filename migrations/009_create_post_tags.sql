-- 009: Create post_tags junction table
-- Migration: Many-to-many relationship between posts and tags

CREATE TABLE post_tags (
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (post_id, tag_id)
);

-- Create index for tag lookups
CREATE INDEX idx_post_tags_tag ON post_tags(tag_id);
