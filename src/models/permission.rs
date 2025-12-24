//! Permission model for fine-grained access control.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// Permission entity from database.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub resource: String,
    pub action: String,
    pub created_at: DateTime<Utc>,
}

/// Common permission names as constants.
pub mod permissions {
    // Posts
    pub const POSTS_READ: &str = "posts:read";
    pub const POSTS_CREATE: &str = "posts:create";
    pub const POSTS_UPDATE: &str = "posts:update";
    pub const POSTS_DELETE: &str = "posts:delete";
    pub const POSTS_PUBLISH: &str = "posts:publish";

    // Categories
    pub const CATEGORIES_READ: &str = "categories:read";
    pub const CATEGORIES_CREATE: &str = "categories:create";
    pub const CATEGORIES_UPDATE: &str = "categories:update";
    pub const CATEGORIES_DELETE: &str = "categories:delete";

    // Tags
    pub const TAGS_READ: &str = "tags:read";
    pub const TAGS_CREATE: &str = "tags:create";
    pub const TAGS_UPDATE: &str = "tags:update";
    pub const TAGS_DELETE: &str = "tags:delete";

    // Users
    pub const USERS_READ: &str = "users:read";
    pub const USERS_CREATE: &str = "users:create";
    pub const USERS_UPDATE: &str = "users:update";
    pub const USERS_DELETE: &str = "users:delete";
}

#[cfg(test)]
mod tests {
    use super::permissions::*;

    #[test]
    fn test_permission_names() {
        assert_eq!(POSTS_READ, "posts:read");
        assert_eq!(POSTS_PUBLISH, "posts:publish");
        assert_eq!(CATEGORIES_CREATE, "categories:create");
    }
}
