//! Permission controller for permission management.

use axum::{extract::State, Json};
use sqlx::PgPool;

use crate::error::AppError;
use crate::models::Permission;
use crate::response::{success, ApiResponse};

/// List all permissions.
pub async fn list_permissions(
    State(pool): State<PgPool>,
) -> Result<Json<ApiResponse<Vec<Permission>>>, AppError> {
    let permissions = sqlx::query_as::<_, Permission>(
        r#"
        SELECT id, name, description, resource, action, created_at
        FROM permissions
        ORDER BY resource, action
        "#,
    )
    .fetch_all(&pool)
    .await?;

    Ok(success(permissions))
}
