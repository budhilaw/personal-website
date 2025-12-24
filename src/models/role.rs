//! Role model for dynamic RBAC.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Role entity from database.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Role response (without soft delete field).
#[derive(Debug, Clone, Serialize)]
pub struct RoleResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Role> for RoleResponse {
    fn from(role: Role) -> Self {
        Self {
            id: role.id,
            name: role.name,
            slug: role.slug,
            description: role.description,
            created_at: role.created_at,
        }
    }
}

/// Request payload for creating a role.
#[derive(Debug, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
}

/// Request payload for updating a role.
#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
}

/// Common role slugs (for convenience, not enforcement).
pub mod slugs {
    pub const ADMIN: &str = "admin";
    pub const EDITOR: &str = "editor";
    pub const WRITER: &str = "writer";
    pub const VIEWER: &str = "viewer";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_slugs() {
        assert_eq!(slugs::ADMIN, "admin");
        assert_eq!(slugs::EDITOR, "editor");
        assert_eq!(slugs::WRITER, "writer");
        assert_eq!(slugs::VIEWER, "viewer");
    }
}
