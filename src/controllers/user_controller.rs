//! User controller for user management (admin only).

use axum::{
    extract::{Path, State},
    Extension, Json,
};
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::AuthUser;
use crate::models::{CreateUserRequest, UserWithRoleResponse};
use crate::repositories::UserRepository;
use crate::response::{success, ApiResponse, MessageResponse};
use crate::services::AuthService;

/// List all users (admin only).
pub async fn list_users(
    State(user_repo): State<UserRepository>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<ApiResponse<Vec<UserWithRoleResponse>>>, AppError> {
    if !auth_user.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    let users = user_repo.find_all().await?;
    let responses: Vec<UserWithRoleResponse> = users.into_iter().map(|u| u.into()).collect();
    Ok(success(responses))
}

/// Get a user by ID (admin only).
pub async fn get_user(
    State(user_repo): State<UserRepository>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserWithRoleResponse>>, AppError> {
    if !auth_user.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    let user = user_repo
        .find_by_id_with_role(id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
    Ok(success(user.into()))
}

/// Create a new user (admin only).
pub async fn create_user(
    State(auth_service): State<AuthService>,
    State(user_repo): State<UserRepository>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<UserWithRoleResponse>>, AppError> {
    if !auth_user.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    // Hash password
    let password_hash = auth_service.hash_password(&request.password)?;

    // Default to viewer role if not specified
    let role_id = request
        .role_id
        .ok_or_else(|| AppError::ValidationError("role_id is required".to_string()))?;

    let user = user_repo
        .create(&request.email, &password_hash, &request.name, role_id)
        .await?;

    // Fetch with role info
    let user_with_role = user_repo
        .find_by_id_with_role(user.id)
        .await?
        .ok_or_else(|| AppError::InternalError("Failed to fetch created user".to_string()))?;

    Ok(success(user_with_role.into()))
}

/// Delete a user (admin only).
pub async fn delete_user(
    State(user_repo): State<UserRepository>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<MessageResponse>>, AppError> {
    if !auth_user.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    // Prevent self-deletion
    if auth_user.id == id {
        return Err(AppError::ValidationError(
            "Cannot delete yourself".to_string(),
        ));
    }

    user_repo.delete(id).await?;
    Ok(success(MessageResponse::new("User deleted successfully")))
}
