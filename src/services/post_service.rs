//! Post service for blog post business logic.

use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    AuthorResponse, Category, CreatePostRequest, Post, PostListItem, PostQuery, PostResponse,
    PostStatus, Tag, UpdatePostRequest,
};
use crate::repositories::{CategoryRepository, PostRepository, TagRepository, UserRepository};
use crate::response::Meta;

/// Service for blog post operations.
#[derive(Clone)]
pub struct PostService {
    post_repo: PostRepository,
    user_repo: UserRepository,
    category_repo: CategoryRepository,
    tag_repo: TagRepository,
}

impl PostService {
    /// Create a new post service.
    pub fn new(
        post_repo: PostRepository,
        user_repo: UserRepository,
        category_repo: CategoryRepository,
        tag_repo: TagRepository,
    ) -> Self {
        Self {
            post_repo,
            user_repo,
            category_repo,
            tag_repo,
        }
    }

    /// List posts with pagination and filters.
    pub async fn list(
        &self,
        query: PostQuery,
        is_admin: bool,
    ) -> Result<(Vec<PostListItem>, Meta), AppError> {
        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.per_page.unwrap_or(10).clamp(1, 100);
        let offset = (page - 1) * per_page;

        // Non-admin users can only see published posts
        let status = if is_admin {
            query.status
        } else {
            Some(PostStatus::Published)
        };

        let posts = self
            .post_repo
            .find_all(status, query.category_id, per_page, offset)
            .await?;

        let total = self.post_repo.count(status, query.category_id).await?;

        Ok((posts, Meta::new(page, per_page, total)))
    }

    /// Get a single post by slug.
    pub async fn get_by_slug(&self, slug: &str, is_admin: bool) -> Result<PostResponse, AppError> {
        let post = self
            .post_repo
            .find_by_slug(slug)
            .await?
            .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

        // Non-admin users can only see published posts
        if !is_admin && post.status != PostStatus::Published {
            return Err(AppError::NotFound("Post not found".to_string()));
        }

        self.build_post_response(post).await
    }

    /// Get a single post by ID.
    pub async fn get_by_id(&self, id: Uuid, is_admin: bool) -> Result<PostResponse, AppError> {
        let post = self
            .post_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

        // Non-admin users can only see published posts
        if !is_admin && post.status != PostStatus::Published {
            return Err(AppError::NotFound("Post not found".to_string()));
        }

        self.build_post_response(post).await
    }

    /// Create a new post.
    pub async fn create(
        &self,
        author_id: Uuid,
        request: CreatePostRequest,
    ) -> Result<PostResponse, AppError> {
        // Generate slug if not provided
        let slug = request
            .slug
            .unwrap_or_else(|| Self::slugify(&request.title));

        // Check if slug already exists
        if self.post_repo.find_by_slug(&slug).await?.is_some() {
            return Err(AppError::Conflict("Slug already exists".to_string()));
        }

        let post = self
            .post_repo
            .create(
                &request.title,
                &slug,
                &request.content,
                request.excerpt.as_deref(),
                request.status.unwrap_or_default(),
                author_id,
                request.category_id,
            )
            .await?;

        // Set tags if provided
        if let Some(tag_ids) = request.tag_ids {
            self.post_repo.set_tags(post.id, &tag_ids).await?;
        }

        self.build_post_response(post).await
    }

    /// Update an existing post.
    pub async fn update(
        &self,
        id: Uuid,
        request: UpdatePostRequest,
    ) -> Result<PostResponse, AppError> {
        // Check if post exists
        self.post_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

        // Check slug uniqueness if updating
        if let Some(ref slug) = request.slug {
            if let Some(existing) = self.post_repo.find_by_slug(slug).await? {
                if existing.id != id {
                    return Err(AppError::Conflict("Slug already exists".to_string()));
                }
            }
        }

        let post = self
            .post_repo
            .update(
                id,
                request.title.as_deref(),
                request.slug.as_deref(),
                request.content.as_deref(),
                request.excerpt.as_deref(),
                request.status,
                request.category_id,
            )
            .await?;

        // Update tags if provided
        if let Some(tag_ids) = request.tag_ids {
            self.post_repo.set_tags(post.id, &tag_ids).await?;
        }

        self.build_post_response(post).await
    }

    /// Delete a post.
    pub async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        self.post_repo.delete(id).await
    }

    // Private helper methods

    async fn build_post_response(&self, post: Post) -> Result<PostResponse, AppError> {
        // Get author
        let author: Option<AuthorResponse> = self
            .user_repo
            .find_by_id(post.author_id)
            .await?
            .map(|user| user.into());

        // Get category
        let category: Option<Category> = if let Some(cat_id) = post.category_id {
            self.category_repo.find_by_id(cat_id).await?
        } else {
            None
        };

        // Get tags
        let tag_ids = self.post_repo.get_tag_ids(post.id).await?;
        let tags: Vec<Tag> = if !tag_ids.is_empty() {
            self.tag_repo.find_by_ids(&tag_ids).await?
        } else {
            vec![]
        };

        Ok(PostResponse {
            id: post.id,
            title: post.title,
            slug: post.slug,
            content: post.content,
            excerpt: post.excerpt,
            status: post.status,
            author,
            category,
            tags,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
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
        assert_eq!(PostService::slugify("Hello World"), "hello-world");
        assert_eq!(PostService::slugify("Hello  World"), "hello-world");
        assert_eq!(PostService::slugify("Hello World!"), "hello-world");
        assert_eq!(PostService::slugify("  Hello   World  "), "hello-world");
        assert_eq!(PostService::slugify("Rust 2024"), "rust-2024");
    }
}
