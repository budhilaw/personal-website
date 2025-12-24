-- 007: Create post_status enum
-- Migration: Post status enum type

CREATE TYPE post_status AS ENUM ('draft', 'published', 'archived');
