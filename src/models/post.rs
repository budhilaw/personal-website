//! Blog post model and status definitions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{Category, Tag, User};

/// Post status enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, sqlx::Type)]
#[sqlx(type_name = "post_status", rename_all = "lowercase")]
pub enum PostStatus {
    #[default]
    Draft,
    Published,
    Archived,
}

impl std::fmt::Display for PostStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostStatus::Draft => write!(f, "draft"),
            PostStatus::Published => write!(f, "published"),
            PostStatus::Archived => write!(f, "archived"),
        }
    }
}

/// Post entity from database.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub status: PostStatus,
    pub author_id: Uuid,
    pub category_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Simple author info for post responses.
#[derive(Debug, Clone, Serialize)]
pub struct AuthorResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl From<User> for AuthorResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}

/// Post with relations for API response.
#[derive(Debug, Clone, Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub status: PostStatus,
    pub author: Option<AuthorResponse>,
    pub category: Option<Category>,
    pub tags: Vec<Tag>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Post list item (lighter version for lists).
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PostListItem {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub status: PostStatus,
    pub author_id: Uuid,
    pub author_name: Option<String>,
    pub category_id: Option<Uuid>,
    pub category_name: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Request payload for creating a post.
#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub slug: Option<String>,
    pub content: String,
    pub excerpt: Option<String>,
    pub status: Option<PostStatus>,
    pub category_id: Option<Uuid>,
    pub tag_ids: Option<Vec<Uuid>>,
}

/// Request payload for updating a post.
#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub status: Option<PostStatus>,
    pub category_id: Option<Uuid>,
    pub tag_ids: Option<Vec<Uuid>>,
}

/// Query parameters for listing posts.
#[derive(Debug, Deserialize)]
pub struct PostQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<PostStatus>,
    pub category_id: Option<Uuid>,
    pub tag_id: Option<Uuid>,
    pub search: Option<String>,
}

impl Default for PostQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(10),
            status: None,
            category_id: None,
            tag_id: None,
            search: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_status_display() {
        assert_eq!(PostStatus::Draft.to_string(), "draft");
        assert_eq!(PostStatus::Published.to_string(), "published");
        assert_eq!(PostStatus::Archived.to_string(), "archived");
    }

    #[test]
    fn test_post_status_default() {
        assert_eq!(PostStatus::default(), PostStatus::Draft);
    }

    #[test]
    fn test_post_query_default() {
        let query = PostQuery::default();
        assert_eq!(query.page, Some(1));
        assert_eq!(query.per_page, Some(10));
        assert!(query.status.is_none());
    }
}
