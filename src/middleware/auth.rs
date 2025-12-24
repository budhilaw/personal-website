//! Authentication middleware for JWT validation and permission-based RBAC.

use axum::http::header;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::error::AppError;
use crate::routes::AppState;
use crate::services::Claims;

/// Authenticated user information extracted from JWT.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub role_id: Uuid,
    pub role_slug: String,
    /// Cached permissions (loaded on first check)
    pub permissions: Vec<String>,
}

impl AuthUser {
    /// Check if user has admin role.
    pub fn is_admin(&self) -> bool {
        self.role_slug == "admin"
    }

    /// Check if user has a specific permission.
    pub fn has_permission(&self, permission: &str) -> bool {
        // Admin has all permissions
        if self.is_admin() {
            return true;
        }
        self.permissions.iter().any(|p| p == permission)
    }

    /// Check if user can create resources of a given type.
    pub fn can_create(&self, resource: &str) -> bool {
        self.has_permission(&format!("{}:create", resource))
    }

    /// Check if user can read resources of a given type.
    pub fn can_read(&self, resource: &str) -> bool {
        self.has_permission(&format!("{}:read", resource))
    }

    /// Check if user can update resources of a given type.
    pub fn can_update(&self, resource: &str) -> bool {
        self.has_permission(&format!("{}:update", resource))
    }

    /// Check if user can delete resources of a given type.
    pub fn can_delete(&self, resource: &str) -> bool {
        self.has_permission(&format!("{}:delete", resource))
    }

    /// Check if user can publish posts.
    pub fn can_publish(&self) -> bool {
        self.has_permission("posts:publish")
    }
}

/// Extract bearer token from Authorization header.
fn extract_bearer_token(request: &Request) -> Option<String> {
    request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer ").map(|s| s.to_string()))
}

/// Create AuthUser from Claims and load permissions.
async fn create_auth_user(claims: &Claims, state: &AppState) -> Result<AuthUser, AppError> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::JwtError("Invalid user ID in token".to_string()))?;
    let role_id = Uuid::parse_str(&claims.role_id)
        .map_err(|_| AppError::JwtError("Invalid role ID in token".to_string()))?;

    // Load permissions for this role
    let permissions = state.auth_service.get_user_permissions(role_id).await?;

    Ok(AuthUser {
        id: user_id,
        email: claims.email.clone(),
        role_id,
        role_slug: claims.role_slug.clone(),
        permissions,
    })
}

/// Authentication middleware - requires valid JWT token.
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_bearer_token(&request).ok_or(AppError::Unauthorized)?;
    let claims = state.auth_service.validate_access_token(&token).await?;
    let auth_user = create_auth_user(&claims, &state).await?;

    request.extensions_mut().insert(auth_user);
    Ok(next.run(request).await)
}

/// Admin-only middleware - requires admin role.
pub async fn admin_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_bearer_token(&request).ok_or(AppError::Unauthorized)?;
    let claims = state.auth_service.validate_access_token(&token).await?;
    let auth_user = create_auth_user(&claims, &state).await?;

    if !auth_user.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    request.extensions_mut().insert(auth_user);
    Ok(next.run(request).await)
}

/// Optional auth middleware - extracts user if token present, continues if not.
pub async fn optional_auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let auth_user: Option<AuthUser> = if let Some(token) = extract_bearer_token(&request) {
        if let Ok(claims) = state.auth_service.validate_access_token(&token).await {
            create_auth_user(&claims, &state).await.ok()
        } else {
            None
        }
    } else {
        None
    };

    request.extensions_mut().insert(auth_user);
    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_user_permissions() {
        let admin = AuthUser {
            id: Uuid::new_v4(),
            email: "admin@test.com".to_string(),
            role_id: Uuid::new_v4(),
            role_slug: "admin".to_string(),
            permissions: vec![],
        };
        assert!(admin.is_admin());
        assert!(admin.has_permission("anything")); // Admin has all permissions

        let writer = AuthUser {
            id: Uuid::new_v4(),
            email: "writer@test.com".to_string(),
            role_id: Uuid::new_v4(),
            role_slug: "writer".to_string(),
            permissions: vec![
                "posts:read".to_string(),
                "posts:create".to_string(),
                "posts:update".to_string(),
            ],
        };
        assert!(!writer.is_admin());
        assert!(writer.can_create("posts"));
        assert!(writer.can_read("posts"));
        assert!(writer.can_update("posts"));
        assert!(!writer.can_delete("posts"));
        assert!(!writer.can_publish());
    }

    #[test]
    fn test_auth_user_is_admin() {
        let admin = AuthUser {
            id: Uuid::new_v4(),
            email: "admin@test.com".to_string(),
            role_id: Uuid::new_v4(),
            role_slug: "admin".to_string(),
            permissions: vec![],
        };
        assert!(admin.is_admin());

        let viewer = AuthUser {
            id: Uuid::new_v4(),
            email: "viewer@test.com".to_string(),
            role_id: Uuid::new_v4(),
            role_slug: "viewer".to_string(),
            permissions: vec!["posts:read".to_string()],
        };
        assert!(!viewer.is_admin());
    }
}
