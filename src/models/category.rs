//! Category model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Category entity from database.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request payload for creating a category.
#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
}

/// Request payload for updating a category.
#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
}

/// Category with post count for listing.
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct CategoryWithCount {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub post_count: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_serialization() {
        let category = Category {
            id: Uuid::new_v4(),
            name: "Technology".to_string(),
            slug: "technology".to_string(),
            description: Some("Tech posts".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&category).unwrap();
        assert!(json.contains("Technology"));
        assert!(json.contains("technology"));
    }

    #[test]
    fn test_create_category_request() {
        let json = r#"{"name": "Tech", "slug": "tech"}"#;
        let request: CreateCategoryRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.name, "Tech");
        assert_eq!(request.slug, Some("tech".to_string()));
    }
}
