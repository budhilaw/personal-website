//! Category service for category business logic.

use uuid::Uuid;

use crate::error::AppError;
use crate::models::{Category, CategoryWithCount, CreateCategoryRequest, UpdateCategoryRequest};
use crate::repositories::CategoryRepository;

/// Service for category operations.
#[derive(Clone)]
pub struct CategoryService {
    repo: CategoryRepository,
}

impl CategoryService {
    /// Create a new category service.
    pub fn new(repo: CategoryRepository) -> Self {
        Self { repo }
    }

    /// List all categories with post counts.
    pub async fn list(&self) -> Result<Vec<CategoryWithCount>, AppError> {
        self.repo.find_all_with_count().await
    }

    /// Get a single category by ID.
    pub async fn get_by_id(&self, id: Uuid) -> Result<Category, AppError> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Category not found".to_string()))
    }

    /// Get a single category by slug.
    pub async fn get_by_slug(&self, slug: &str) -> Result<Category, AppError> {
        self.repo
            .find_by_slug(slug)
            .await?
            .ok_or_else(|| AppError::NotFound("Category not found".to_string()))
    }

    /// Create a new category.
    pub async fn create(&self, request: CreateCategoryRequest) -> Result<Category, AppError> {
        let slug = request.slug.unwrap_or_else(|| Self::slugify(&request.name));

        // Check if slug already exists
        if self.repo.find_by_slug(&slug).await?.is_some() {
            return Err(AppError::Conflict(
                "Category slug already exists".to_string(),
            ));
        }

        self.repo
            .create(&request.name, &slug, request.description.as_deref())
            .await
    }

    /// Update an existing category.
    pub async fn update(
        &self,
        id: Uuid,
        request: UpdateCategoryRequest,
    ) -> Result<Category, AppError> {
        // Check if category exists
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

        // Check slug uniqueness if updating
        if let Some(ref slug) = request.slug {
            if let Some(existing) = self.repo.find_by_slug(slug).await? {
                if existing.id != id {
                    return Err(AppError::Conflict(
                        "Category slug already exists".to_string(),
                    ));
                }
            }
        }

        self.repo
            .update(
                id,
                request.name.as_deref(),
                request.slug.as_deref(),
                request.description.as_deref(),
            )
            .await
    }

    /// Delete a category.
    pub async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        // Check if category exists
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

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
        assert_eq!(CategoryService::slugify("Technology"), "technology");
        assert_eq!(
            CategoryService::slugify("Web Development"),
            "web-development"
        );
        assert_eq!(CategoryService::slugify("Rust & Go"), "rust-go");
    }
}
