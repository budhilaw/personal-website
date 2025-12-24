//! Role repository for database operations.

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::Role;

/// Repository for role database operations.
#[derive(Clone)]
pub struct RoleRepository {
    pool: PgPool,
}

impl RoleRepository {
    /// Create a new role repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Find a role by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>, AppError> {
        let role = sqlx::query_as::<_, Role>(
            r#"
            SELECT id, name, slug, description, created_at, updated_at, deleted_at
            FROM roles
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(role)
    }

    /// Find a role by slug.
    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<Role>, AppError> {
        let role = sqlx::query_as::<_, Role>(
            r#"
            SELECT id, name, slug, description, created_at, updated_at, deleted_at
            FROM roles
            WHERE slug = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(role)
    }

    /// Get all roles.
    pub async fn find_all(&self) -> Result<Vec<Role>, AppError> {
        let roles = sqlx::query_as::<_, Role>(
            r#"
            SELECT id, name, slug, description, created_at, updated_at, deleted_at
            FROM roles
            WHERE deleted_at IS NULL
            ORDER BY name ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(roles)
    }

    /// Create a new role.
    pub async fn create(
        &self,
        name: &str,
        slug: &str,
        description: Option<&str>,
    ) -> Result<Role, AppError> {
        let role = sqlx::query_as::<_, Role>(
            r#"
            INSERT INTO roles (name, slug, description)
            VALUES ($1, $2, $3)
            RETURNING id, name, slug, description, created_at, updated_at, deleted_at
            "#,
        )
        .bind(name)
        .bind(slug)
        .bind(description)
        .fetch_one(&self.pool)
        .await?;

        Ok(role)
    }

    /// Update a role.
    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        slug: Option<&str>,
        description: Option<&str>,
    ) -> Result<Role, AppError> {
        let role = sqlx::query_as::<_, Role>(
            r#"
            UPDATE roles
            SET 
                name = COALESCE($2, name),
                slug = COALESCE($3, slug),
                description = COALESCE($4, description)
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, name, slug, description, created_at, updated_at, deleted_at
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(slug)
        .bind(description)
        .fetch_one(&self.pool)
        .await?;

        Ok(role)
    }

    /// Soft delete a role.
    pub async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE roles SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Get permissions for a role.
    pub async fn get_permissions(&self, role_id: Uuid) -> Result<Vec<String>, AppError> {
        let permissions: Vec<(String,)> = sqlx::query_as(
            r#"
            SELECT p.name
            FROM permissions p
            JOIN role_permissions rp ON p.id = rp.permission_id
            WHERE rp.role_id = $1
            "#,
        )
        .bind(role_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(permissions.into_iter().map(|(name,)| name).collect())
    }

    /// Assign a permission to a role.
    pub async fn assign_permission(
        &self,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            INSERT INTO role_permissions (role_id, permission_id)
            VALUES ($1, $2)
            ON CONFLICT (role_id, permission_id) DO NOTHING
            "#,
        )
        .bind(role_id)
        .bind(permission_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Remove a permission from a role.
    pub async fn remove_permission(
        &self,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM role_permissions
            WHERE role_id = $1 AND permission_id = $2
            "#,
        )
        .bind(role_id)
        .bind(permission_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
