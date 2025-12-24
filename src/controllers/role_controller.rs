//! Role controller for role management (admin only).

use axum::{
    extract::{Path, State},
    Extension, Json,
};
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::AuthUser;
use crate::models::{CreateRoleRequest, RoleResponse, UpdateRoleRequest};
use crate::repositories::RoleRepository;
use crate::response::{success, ApiResponse, MessageResponse};

/// List all roles.
pub async fn list_roles(
    State(role_repo): State<RoleRepository>,
) -> Result<Json<ApiResponse<Vec<RoleResponse>>>, AppError> {
    let roles = role_repo.find_all().await?;
    let responses: Vec<RoleResponse> = roles.into_iter().map(|r| r.into()).collect();
    Ok(success(responses))
}

/// Get a role by ID.
pub async fn get_role(
    State(role_repo): State<RoleRepository>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<RoleResponse>>, AppError> {
    let role = role_repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;
    Ok(success(role.into()))
}

/// Get permissions for a role.
pub async fn get_role_permissions(
    State(role_repo): State<RoleRepository>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<String>>>, AppError> {
    // Verify role exists
    role_repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

    let permissions = role_repo.get_permissions(id).await?;
    Ok(success(permissions))
}

/// Create a new role (admin only).
pub async fn create_role(
    State(role_repo): State<RoleRepository>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<CreateRoleRequest>,
) -> Result<Json<ApiResponse<RoleResponse>>, AppError> {
    if !auth_user.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    // Generate slug if not provided
    let slug = request.slug.unwrap_or_else(|| slugify(&request.name));

    // Check if slug exists
    if role_repo.find_by_slug(&slug).await?.is_some() {
        return Err(AppError::Conflict("Role slug already exists".to_string()));
    }

    let role = role_repo
        .create(&request.name, &slug, request.description.as_deref())
        .await?;

    Ok(success(role.into()))
}

/// Update a role (admin only).
pub async fn update_role(
    State(role_repo): State<RoleRepository>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateRoleRequest>,
) -> Result<Json<ApiResponse<RoleResponse>>, AppError> {
    if !auth_user.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    // Check if role exists
    role_repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

    // Check slug uniqueness if updating
    if let Some(ref slug) = request.slug {
        if let Some(existing) = role_repo.find_by_slug(slug).await? {
            if existing.id != id {
                return Err(AppError::Conflict("Role slug already exists".to_string()));
            }
        }
    }

    let role = role_repo
        .update(
            id,
            request.name.as_deref(),
            request.slug.as_deref(),
            request.description.as_deref(),
        )
        .await?;

    Ok(success(role.into()))
}

/// Delete a role (admin only).
pub async fn delete_role(
    State(role_repo): State<RoleRepository>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<MessageResponse>>, AppError> {
    if !auth_user.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    // Prevent deleting built-in roles
    let role = role_repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

    if ["admin", "editor", "writer", "viewer"].contains(&role.slug.as_str()) {
        return Err(AppError::ValidationError(
            "Cannot delete built-in roles".to_string(),
        ));
    }

    role_repo.delete(id).await?;
    Ok(success(MessageResponse::new("Role deleted successfully")))
}

/// Request payload for assigning permission to a role.
#[derive(Debug, serde::Deserialize)]
pub struct AssignPermissionRequest {
    pub permission_id: Uuid,
}

/// Assign a permission to a role (admin only).
pub async fn assign_permission(
    State(role_repo): State<RoleRepository>,
    Extension(auth_user): Extension<AuthUser>,
    Path(role_id): Path<Uuid>,
    Json(request): Json<AssignPermissionRequest>,
) -> Result<Json<ApiResponse<MessageResponse>>, AppError> {
    if !auth_user.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    // Verify role exists
    role_repo
        .find_by_id(role_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

    let assigned = role_repo
        .assign_permission(role_id, request.permission_id)
        .await?;

    if assigned {
        Ok(success(MessageResponse::new("Permission assigned to role")))
    } else {
        Ok(success(MessageResponse::new(
            "Permission already assigned to role",
        )))
    }
}

/// Remove a permission from a role (admin only).
pub async fn remove_permission(
    State(role_repo): State<RoleRepository>,
    Extension(auth_user): Extension<AuthUser>,
    Path((role_id, permission_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<MessageResponse>>, AppError> {
    if !auth_user.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    // Verify role exists
    role_repo
        .find_by_id(role_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

    let removed = role_repo.remove_permission(role_id, permission_id).await?;

    if removed {
        Ok(success(MessageResponse::new(
            "Permission removed from role",
        )))
    } else {
        Ok(success(MessageResponse::new(
            "Permission was not assigned to role",
        )))
    }
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
