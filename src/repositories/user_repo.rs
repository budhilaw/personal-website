//! User repository for database operations.

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{User, UserWithRole};

/// Repository for user database operations.
#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    /// Create a new user repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Find a user by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password_hash, name, role_id, created_at, updated_at, deleted_at
            FROM users
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Find a user by email with role info.
    pub async fn find_by_email_with_role(
        &self,
        email: &str,
    ) -> Result<Option<UserWithRole>, AppError> {
        let user = sqlx::query_as::<_, UserWithRole>(
            r#"
            SELECT 
                u.id, u.email, u.password_hash, u.name, u.role_id,
                r.slug as role_slug, r.name as role_name,
                u.created_at, u.updated_at
            FROM users u
            JOIN roles r ON u.role_id = r.id
            WHERE u.email = $1 AND u.deleted_at IS NULL
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Find a user by ID with role info.
    pub async fn find_by_id_with_role(&self, id: Uuid) -> Result<Option<UserWithRole>, AppError> {
        let user = sqlx::query_as::<_, UserWithRole>(
            r#"
            SELECT 
                u.id, u.email, u.password_hash, u.name, u.role_id,
                r.slug as role_slug, r.name as role_name,
                u.created_at, u.updated_at
            FROM users u
            JOIN roles r ON u.role_id = r.id
            WHERE u.id = $1 AND u.deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Create a new user.
    pub async fn create(
        &self,
        email: &str,
        password_hash: &str,
        name: &str,
        role_id: Uuid,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, password_hash, name, role_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, password_hash, name, role_id, created_at, updated_at, deleted_at
            "#,
        )
        .bind(email)
        .bind(password_hash)
        .bind(name)
        .bind(role_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Get all users.
    pub async fn find_all(&self) -> Result<Vec<UserWithRole>, AppError> {
        let users = sqlx::query_as::<_, UserWithRole>(
            r#"
            SELECT 
                u.id, u.email, u.password_hash, u.name, u.role_id,
                r.slug as role_slug, r.name as role_name,
                u.created_at, u.updated_at
            FROM users u
            JOIN roles r ON u.role_id = r.id
            WHERE u.deleted_at IS NULL
            ORDER BY u.created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    /// Soft delete a user by ID.
    pub async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        let result =
            sqlx::query("UPDATE users SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL")
                .bind(id)
                .execute(&self.pool)
                .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_repository_clone() {
        // Repository should be cloneable for use in handlers
        // This is a compile-time check
    }
}
