//! Category repository for database operations.

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{Category, CategoryWithCount};

/// Repository for category database operations.
#[derive(Clone)]
pub struct CategoryRepository {
    pool: PgPool,
}

impl CategoryRepository {
    /// Create a new category repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Find a category by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, AppError> {
        let category = sqlx::query_as::<_, Category>(
            r#"
            SELECT id, name, slug, description, created_at, updated_at
            FROM categories
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(category)
    }

    /// Find a category by slug.
    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<Category>, AppError> {
        let category = sqlx::query_as::<_, Category>(
            r#"
            SELECT id, name, slug, description, created_at, updated_at
            FROM categories
            WHERE slug = $1
            "#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(category)
    }

    /// Find all categories with post counts.
    pub async fn find_all_with_count(&self) -> Result<Vec<CategoryWithCount>, AppError> {
        let categories = sqlx::query_as::<_, CategoryWithCount>(
            r#"
            SELECT 
                c.id, c.name, c.slug, c.description,
                COUNT(p.id) as post_count, c.created_at
            FROM categories c
            LEFT JOIN posts p ON c.id = p.category_id AND p.status = 'published'
            GROUP BY c.id
            ORDER BY c.name ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }

    /// Create a new category.
    pub async fn create(
        &self,
        name: &str,
        slug: &str,
        description: Option<&str>,
    ) -> Result<Category, AppError> {
        let category = sqlx::query_as::<_, Category>(
            r#"
            INSERT INTO categories (name, slug, description)
            VALUES ($1, $2, $3)
            RETURNING id, name, slug, description, created_at, updated_at
            "#,
        )
        .bind(name)
        .bind(slug)
        .bind(description)
        .fetch_one(&self.pool)
        .await?;

        Ok(category)
    }

    /// Update a category.
    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        slug: Option<&str>,
        description: Option<&str>,
    ) -> Result<Category, AppError> {
        let category = sqlx::query_as::<_, Category>(
            r#"
            UPDATE categories
            SET 
                name = COALESCE($2, name),
                slug = COALESCE($3, slug),
                description = COALESCE($4, description)
            WHERE id = $1
            RETURNING id, name, slug, description, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(slug)
        .bind(description)
        .fetch_one(&self.pool)
        .await?;

        Ok(category)
    }

    /// Delete a category by ID.
    pub async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM categories WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
