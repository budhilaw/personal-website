//! Authentication controller for login, refresh, and logout.

use axum::{extract::State, Extension, Json};

use crate::error::AppError;
use crate::middleware::AuthUser;
use crate::models::{LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse};
use crate::response::{success, ApiResponse, MessageResponse};
use crate::services::AuthService;

/// Login endpoint.
pub async fn login(
    State(auth_service): State<AuthService>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    let response = auth_service
        .login(&request.email, &request.password)
        .await?;
    Ok(success(response))
}

/// Refresh token endpoint.
pub async fn refresh_token(
    State(auth_service): State<AuthService>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<Json<ApiResponse<RefreshTokenResponse>>, AppError> {
    let response = auth_service.refresh_token(&request.refresh_token).await?;
    Ok(success(response))
}

/// Logout endpoint - requires authentication.
pub async fn logout(
    State(auth_service): State<AuthService>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<ApiResponse<MessageResponse>>, AppError> {
    auth_service.logout(auth_user.id).await?;
    Ok(success(MessageResponse::new("Successfully logged out")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"email": "test@example.com", "password": "secret123"}"#;
        let request: LoginRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.email, "test@example.com");
        assert_eq!(request.password, "secret123");
    }

    #[test]
    fn test_refresh_token_request_deserialization() {
        let json = r#"{"refresh_token": "some-token"}"#;
        let request: RefreshTokenRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.refresh_token, "some-token");
    }
}
