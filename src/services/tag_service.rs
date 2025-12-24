//! Tag service for tag business logic.

use uuid::Uuid;

use crate::error::AppError;
use crate::models::{CreateTagRequest, Tag, TagWithCount, UpdateTagRequest};
use crate::repositories::TagRepository;

/// Service for tag operations.
#[derive(Clone)]
pub struct TagService {
    repo: TagRepository,
}

impl TagService {
    /// Create a new tag service.
    pub fn new(repo: TagRepository) -> Self {
        Self { repo }
    }

    /// List all tags with post counts.
    pub async fn list(&self) -> Result<Vec<TagWithCount>, AppError> {
        self.repo.find_all_with_count().await
    }

    /// Get a single tag by ID.
    pub async fn get_by_id(&self, id: Uuid) -> Result<Tag, AppError> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Tag not found".to_string()))
    }

    /// Get a single tag by slug.
    pub async fn get_by_slug(&self, slug: &str) -> Result<Tag, AppError> {
        self.repo
            .find_by_slug(slug)
            .await?
            .ok_or_else(|| AppError::NotFound("Tag not found".to_string()))
    }

    /// Create a new tag.
    pub async fn create(&self, request: CreateTagRequest) -> Result<Tag, AppError> {
        let slug = request.slug.unwrap_or_else(|| Self::slugify(&request.name));

        // Check if slug already exists
        if self.repo.find_by_slug(&slug).await?.is_some() {
            return Err(AppError::Conflict("Tag slug already exists".to_string()));
        }

        self.repo.create(&request.name, &slug).await
    }

    /// Update an existing tag.
    pub async fn update(&self, id: Uuid, request: UpdateTagRequest) -> Result<Tag, AppError> {
        // Check if tag exists
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Tag not found".to_string()))?;

        // Check slug uniqueness if updating
        if let Some(ref slug) = request.slug {
            if let Some(existing) = self.repo.find_by_slug(slug).await? {
                if existing.id != id {
                    return Err(AppError::Conflict("Tag slug already exists".to_string()));
                }
            }
        }

        self.repo
            .update(id, request.name.as_deref(), request.slug.as_deref())
            .await
    }

    /// Delete a tag.
    pub async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        // Check if tag exists
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Tag not found".to_string()))?;

        self.repo.delete(id).await
    }

    fn slugify(text: &str) -> String {
        text.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(TagService::slugify("Rust"), "rust");
        assert_eq!(TagService::slugify("Web Dev"), "web-dev");
        assert_eq!(TagService::slugify("C++"), "c");
    }
}
