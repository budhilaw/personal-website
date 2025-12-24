//! Tag controller for tag CRUD operations.

use axum::{
    extract::{Path, State},
    Extension, Json,
};
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::AuthUser;
use crate::models::{CreateTagRequest, Tag, TagWithCount, UpdateTagRequest};
use crate::response::{success, ApiResponse, MessageResponse};
use crate::services::TagService;

/// List all tags.
pub async fn list_tags(
    State(tag_service): State<TagService>,
) -> Result<Json<ApiResponse<Vec<TagWithCount>>>, AppError> {
    let tags = tag_service.list().await?;
    Ok(success(tags))
}

/// Get a single tag by ID.
pub async fn get_tag(
    State(tag_service): State<TagService>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Tag>>, AppError> {
    let tag = tag_service.get_by_id(id).await?;
    Ok(success(tag))
}

/// Create a new tag (admin only).
pub async fn create_tag(
    State(tag_service): State<TagService>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<CreateTagRequest>,
) -> Result<Json<ApiResponse<Tag>>, AppError> {
    if !auth_user.can_create("tags") {
        return Err(AppError::Forbidden("Cannot create tags".to_string()));
    }
    let tag = tag_service.create(request).await?;
    Ok(success(tag))
}

/// Update a tag (admin only).
pub async fn update_tag(
    State(tag_service): State<TagService>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateTagRequest>,
) -> Result<Json<ApiResponse<Tag>>, AppError> {
    if !auth_user.can_update("tags") {
        return Err(AppError::Forbidden("Cannot update tags".to_string()));
    }
    let tag = tag_service.update(id, request).await?;
    Ok(success(tag))
}

/// Delete a tag (admin only).
pub async fn delete_tag(
    State(tag_service): State<TagService>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<MessageResponse>>, AppError> {
    if !auth_user.can_delete("tags") {
        return Err(AppError::Forbidden("Cannot delete tags".to_string()));
    }
    tag_service.delete(id).await?;
    Ok(success(MessageResponse::new("Tag deleted successfully")))
}
