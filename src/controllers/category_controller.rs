//! Category controller for category CRUD operations.

use axum::{
    extract::{Path, State},
    Extension, Json,
};
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::AuthUser;
use crate::models::{Category, CategoryWithCount, CreateCategoryRequest, UpdateCategoryRequest};
use crate::response::{success, ApiResponse, MessageResponse};
use crate::services::CategoryService;

/// List all categories.
pub async fn list_categories(
    State(category_service): State<CategoryService>,
) -> Result<Json<ApiResponse<Vec<CategoryWithCount>>>, AppError> {
    let categories = category_service.list().await?;
    Ok(success(categories))
}

/// Get a single category by ID.
pub async fn get_category(
    State(category_service): State<CategoryService>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Category>>, AppError> {
    let category = category_service.get_by_id(id).await?;
    Ok(success(category))
}

/// Create a new category (admin only).
pub async fn create_category(
    State(category_service): State<CategoryService>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<CreateCategoryRequest>,
) -> Result<Json<ApiResponse<Category>>, AppError> {
    if !auth_user.can_create("categories") {
        return Err(AppError::Forbidden("Cannot create categories".to_string()));
    }
    let category = category_service.create(request).await?;
    Ok(success(category))
}

/// Update a category (admin only).
pub async fn update_category(
    State(category_service): State<CategoryService>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateCategoryRequest>,
) -> Result<Json<ApiResponse<Category>>, AppError> {
    if !auth_user.can_update("categories") {
        return Err(AppError::Forbidden("Cannot update categories".to_string()));
    }
    let category = category_service.update(id, request).await?;
    Ok(success(category))
}

/// Delete a category (admin only).
pub async fn delete_category(
    State(category_service): State<CategoryService>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<MessageResponse>>, AppError> {
    if !auth_user.can_delete("categories") {
        return Err(AppError::Forbidden("Cannot delete categories".to_string()));
    }
    category_service.delete(id).await?;
    Ok(success(MessageResponse::new(
        "Category deleted successfully",
    )))
}
