//! Tag repository for database operations.

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{Tag, TagWithCount};

/// Repository for tag database operations.
#[derive(Clone)]
pub struct TagRepository {
    pool: PgPool,
}

impl TagRepository {
    /// Create a new tag repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Find a tag by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Tag>, AppError> {
        let tag = sqlx::query_as::<_, Tag>(
            r#"
            SELECT id, name, slug, created_at
            FROM tags
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(tag)
    }

    /// Find a tag by slug.
    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<Tag>, AppError> {
        let tag = sqlx::query_as::<_, Tag>(
            r#"
            SELECT id, name, slug, created_at
            FROM tags
            WHERE slug = $1
            "#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(tag)
    }

    /// Find tags by IDs.
    pub async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Tag>, AppError> {
        let tags = sqlx::query_as::<_, Tag>(
            r#"
            SELECT id, name, slug, created_at
            FROM tags
            WHERE id = ANY($1)
            "#,
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await?;

        Ok(tags)
    }

    /// Find all tags with post counts.
    pub async fn find_all_with_count(&self) -> Result<Vec<TagWithCount>, AppError> {
        let tags = sqlx::query_as::<_, TagWithCount>(
            r#"
            SELECT 
                t.id, t.name, t.slug,
                COUNT(pt.post_id) as post_count, t.created_at
            FROM tags t
            LEFT JOIN post_tags pt ON t.id = pt.tag_id
            LEFT JOIN posts p ON pt.post_id = p.id AND p.status = 'published'
            GROUP BY t.id
            ORDER BY t.name ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tags)
    }

    /// Create a new tag.
    pub async fn create(&self, name: &str, slug: &str) -> Result<Tag, AppError> {
        let tag = sqlx::query_as::<_, Tag>(
            r#"
            INSERT INTO tags (name, slug)
            VALUES ($1, $2)
            RETURNING id, name, slug, created_at
            "#,
        )
        .bind(name)
        .bind(slug)
        .fetch_one(&self.pool)
        .await?;

        Ok(tag)
    }

    /// Update a tag.
    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        slug: Option<&str>,
    ) -> Result<Tag, AppError> {
        let tag = sqlx::query_as::<_, Tag>(
            r#"
            UPDATE tags
            SET 
                name = COALESCE($2, name),
                slug = COALESCE($3, slug)
            WHERE id = $1
            RETURNING id, name, slug, created_at
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(slug)
        .fetch_one(&self.pool)
        .await?;

        Ok(tag)
    }

    /// Delete a tag by ID.
    pub async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM tags WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
