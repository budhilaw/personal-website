//! Post controller for blog CRUD operations.

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::AuthUser;
use crate::models::{CreatePostRequest, PostListItem, PostQuery, PostResponse, UpdatePostRequest};
use crate::response::{paginated, success, ApiResponse, MessageResponse};
use crate::services::PostService;

/// List posts (public - shows only published, admin - shows all).
pub async fn list_posts(
    State(post_service): State<PostService>,
    Extension(auth_user): Extension<Option<AuthUser>>,
    Query(query): Query<PostQuery>,
) -> Result<Json<ApiResponse<Vec<PostListItem>>>, AppError> {
    let is_admin = auth_user.map(|u| u.is_admin()).unwrap_or(false);
    let (posts, meta) = post_service.list(query, is_admin).await?;
    Ok(paginated(posts, meta.page, meta.per_page, meta.total))
}

/// Get a single post by slug.
pub async fn get_post_by_slug(
    State(post_service): State<PostService>,
    Extension(auth_user): Extension<Option<AuthUser>>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<PostResponse>>, AppError> {
    let is_admin = auth_user.map(|u| u.is_admin()).unwrap_or(false);
    let post = post_service.get_by_slug(&slug, is_admin).await?;
    Ok(success(post))
}

/// Create a new post (admin only).
pub async fn create_post(
    State(post_service): State<PostService>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<CreatePostRequest>,
) -> Result<Json<ApiResponse<PostResponse>>, AppError> {
    if !auth_user.can_create("posts") {
        return Err(AppError::Forbidden("Cannot create posts".to_string()));
    }
    let post = post_service.create(auth_user.id, request).await?;
    Ok(success(post))
}

/// Update a post (admin only).
pub async fn update_post(
    State(post_service): State<PostService>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdatePostRequest>,
) -> Result<Json<ApiResponse<PostResponse>>, AppError> {
    if !auth_user.can_update("posts") {
        return Err(AppError::Forbidden("Cannot update posts".to_string()));
    }
    let post = post_service.update(id, request).await?;
    Ok(success(post))
}

/// Delete a post (admin only).
pub async fn delete_post(
    State(post_service): State<PostService>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<MessageResponse>>, AppError> {
    if !auth_user.can_delete("posts") {
        return Err(AppError::Forbidden("Cannot delete posts".to_string()));
    }
    post_service.delete(id).await?;
    Ok(success(MessageResponse::new("Post deleted successfully")))
}
