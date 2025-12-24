//! Tag model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Tag entity from database.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
}

/// Request payload for creating a tag.
#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub slug: Option<String>,
}

/// Request payload for updating a tag.
#[derive(Debug, Deserialize)]
pub struct UpdateTagRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
}

/// Tag with post count for listing.
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct TagWithCount {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub post_count: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_serialization() {
        let tag = Tag {
            id: Uuid::new_v4(),
            name: "Rust".to_string(),
            slug: "rust".to_string(),
            created_at: Utc::now(),
        };

        let json = serde_json::to_string(&tag).unwrap();
        assert!(json.contains("Rust"));
        assert!(json.contains("rust"));
    }

    #[test]
    fn test_create_tag_request() {
        let json = r#"{"name": "Programming"}"#;
        let request: CreateTagRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.name, "Programming");
        assert!(request.slug.is_none());
    }
}
