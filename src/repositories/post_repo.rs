//! Post repository for database operations.

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{Post, PostListItem, PostStatus};

/// Repository for post database operations.
#[derive(Clone)]
pub struct PostRepository {
    pool: PgPool,
}

impl PostRepository {
    /// Create a new post repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Find a post by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, AppError> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            SELECT id, title, slug, content, excerpt, status, author_id, category_id, created_at, updated_at
            FROM posts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    /// Find a post by slug.
    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<Post>, AppError> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            SELECT id, title, slug, content, excerpt, status, author_id, category_id, created_at, updated_at
            FROM posts
            WHERE slug = $1
            "#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    /// Find all posts with pagination and optional filters.
    pub async fn find_all(
        &self,
        status: Option<PostStatus>,
        category_id: Option<Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PostListItem>, AppError> {
        let posts = sqlx::query_as::<_, PostListItem>(
            r#"
            SELECT 
                p.id, p.title, p.slug, p.excerpt, p.status, p.author_id,
                u.name as author_name, p.category_id, c.name as category_name, p.created_at
            FROM posts p
            LEFT JOIN users u ON p.author_id = u.id
            LEFT JOIN categories c ON p.category_id = c.id
            WHERE ($1::post_status IS NULL OR p.status = $1)
              AND ($2::uuid IS NULL OR p.category_id = $2)
            ORDER BY p.created_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(status)
        .bind(category_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }

    /// Count posts with optional filters.
    pub async fn count(
        &self,
        status: Option<PostStatus>,
        category_id: Option<Uuid>,
    ) -> Result<i64, AppError> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) as count
            FROM posts
            WHERE ($1::post_status IS NULL OR status = $1)
              AND ($2::uuid IS NULL OR category_id = $2)
            "#,
        )
        .bind(status)
        .bind(category_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    /// Create a new post.
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        title: &str,
        slug: &str,
        content: &str,
        excerpt: Option<&str>,
        status: PostStatus,
        author_id: Uuid,
        category_id: Option<Uuid>,
    ) -> Result<Post, AppError> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            INSERT INTO posts (title, slug, content, excerpt, status, author_id, category_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, title, slug, content, excerpt, status, author_id, category_id, created_at, updated_at
            "#,
        )
        .bind(title)
        .bind(slug)
        .bind(content)
        .bind(excerpt)
        .bind(status)
        .bind(author_id)
        .bind(category_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    /// Update a post.
    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        &self,
        id: Uuid,
        title: Option<&str>,
        slug: Option<&str>,
        content: Option<&str>,
        excerpt: Option<&str>,
        status: Option<PostStatus>,
        category_id: Option<Uuid>,
    ) -> Result<Post, AppError> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            UPDATE posts
            SET 
                title = COALESCE($2, title),
                slug = COALESCE($3, slug),
                content = COALESCE($4, content),
                excerpt = COALESCE($5, excerpt),
                status = COALESCE($6, status),
                category_id = COALESCE($7, category_id)
            WHERE id = $1
            RETURNING id, title, slug, content, excerpt, status, author_id, category_id, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(title)
        .bind(slug)
        .bind(content)
        .bind(excerpt)
        .bind(status)
        .bind(category_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    /// Delete a post by ID.
    pub async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM posts WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Get tags for a post.
    pub async fn get_tag_ids(&self, post_id: Uuid) -> Result<Vec<Uuid>, AppError> {
        let tags: Vec<(Uuid,)> = sqlx::query_as("SELECT tag_id FROM post_tags WHERE post_id = $1")
            .bind(post_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(tags.into_iter().map(|(id,)| id).collect())
    }

    /// Set tags for a post (replaces existing).
    pub async fn set_tags(&self, post_id: Uuid, tag_ids: &[Uuid]) -> Result<(), AppError> {
        // Delete existing tags
        sqlx::query("DELETE FROM post_tags WHERE post_id = $1")
            .bind(post_id)
            .execute(&self.pool)
            .await?;

        // Insert new tags
        for tag_id in tag_ids {
            sqlx::query("INSERT INTO post_tags (post_id, tag_id) VALUES ($1, $2)")
                .bind(post_id)
                .bind(tag_id)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }
}
