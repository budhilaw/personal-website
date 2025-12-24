//! User model definitions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::RoleResponse;

/// User entity from database.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    pub role_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// User with role info for API responses.
#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: RoleResponse,
    pub created_at: DateTime<Utc>,
}

/// User with role slug (flattened - useful for JWT claims).
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct UserWithRole {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    pub role_id: Uuid,
    pub role_slug: String,
    pub role_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request payload for creating a user.
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub role_id: Option<Uuid>,
}

/// Request payload for login.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Response payload for login.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserWithRoleResponse,
}

/// User with role for login response.
#[derive(Debug, Clone, Serialize)]
pub struct UserWithRoleResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role_id: Uuid,
    pub role_slug: String,
    pub role_name: String,
}

impl From<UserWithRole> for UserWithRoleResponse {
    fn from(user: UserWithRole) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            role_id: user.role_id,
            role_slug: user.role_slug,
            role_name: user.role_name,
        }
    }
}

/// Request payload for token refresh.
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// Response payload for token refresh.
#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"email":"test@example.com","password":"secret"}"#;
        let req: LoginRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.email, "test@example.com");
        assert_eq!(req.password, "secret");
    }

    #[test]
    fn test_user_with_role_response() {
        let user = UserWithRole {
            id: Uuid::new_v4(),
            email: "test@test.com".to_string(),
            password_hash: "hash".to_string(),
            name: "Test".to_string(),
            role_id: Uuid::new_v4(),
            role_slug: "admin".to_string(),
            role_name: "Administrator".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let response: UserWithRoleResponse = user.into();
        assert_eq!(response.role_slug, "admin");
    }
}
