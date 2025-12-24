//! Authentication service with JWT and password handling.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::Config;
use crate::error::AppError;
use crate::models::{LoginResponse, RefreshTokenResponse, UserWithRole};
use crate::pkg::redis::keys;
use crate::repositories::{RoleRepository, UserRepository};

/// JWT claims structure.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// User email
    pub email: String,
    /// Role ID
    pub role_id: String,
    /// Role slug (for quick permission checks)
    pub role_slug: String,
    /// Token ID for revocation
    pub jti: String,
    /// Expiration time
    pub exp: i64,
    /// Issued at
    pub iat: i64,
    /// Token type (access or refresh)
    pub token_type: String,
}

/// Authentication service.
#[derive(Clone)]
pub struct AuthService {
    config: Config,
    user_repo: UserRepository,
    role_repo: RoleRepository,
    redis: redis::aio::ConnectionManager,
}

impl AuthService {
    /// Create a new auth service.
    pub fn new(
        config: Config,
        user_repo: UserRepository,
        role_repo: RoleRepository,
        redis: redis::aio::ConnectionManager,
    ) -> Self {
        Self {
            config,
            user_repo,
            role_repo,
            redis,
        }
    }

    /// Hash a password using Argon2.
    pub fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))?;
        Ok(hash.to_string())
    }

    /// Verify a password against a hash.
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::InternalError(format!("Invalid password hash: {}", e)))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Login user and return tokens.
    pub async fn login(&self, email: &str, password: &str) -> Result<LoginResponse, AppError> {
        // Find user by email with role
        let user = self
            .user_repo
            .find_by_email_with_role(email)
            .await?
            .ok_or(AppError::Unauthorized)?;

        // Verify password
        if !self.verify_password(password, &user.password_hash)? {
            return Err(AppError::Unauthorized);
        }

        // Generate tokens
        let (access_token, access_jti) = self.create_access_token(&user)?;
        let (refresh_token, refresh_jti) = self.create_refresh_token(&user)?;

        // Store tokens in Redis
        self.store_token(
            &access_jti,
            &user.id,
            "access",
            self.config.jwt_access_expiry_hours * 3600,
        )
        .await?;
        self.store_token(
            &refresh_jti,
            &user.id,
            "refresh",
            self.config.jwt_refresh_expiry_days * 86400,
        )
        .await?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.config.jwt_access_expiry_hours * 3600,
            user: user.into(),
        })
    }

    /// Refresh access token using refresh token.
    pub async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<RefreshTokenResponse, AppError> {
        // Validate refresh token
        let claims = self.validate_token(refresh_token)?;

        if claims.token_type != "refresh" {
            return Err(AppError::JwtError("Invalid token type".to_string()));
        }

        // Check if token is in Redis (not revoked)
        let key = keys::refresh_token(&claims.jti);
        let mut redis = self.redis.clone();
        let exists: bool = redis.exists(&key).await?;
        if !exists {
            return Err(AppError::JwtError("Token has been revoked".to_string()));
        }

        // Get user
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::JwtError("Invalid user ID".to_string()))?;
        let user = self
            .user_repo
            .find_by_id_with_role(user_id)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        // Generate new access token
        let (access_token, access_jti) = self.create_access_token(&user)?;
        self.store_token(
            &access_jti,
            &user.id,
            "access",
            self.config.jwt_access_expiry_hours * 3600,
        )
        .await?;

        Ok(RefreshTokenResponse {
            access_token,
            token_type: "Bearer".to_string(),
            expires_in: self.config.jwt_access_expiry_hours * 3600,
        })
    }

    /// Logout user by revoking all tokens.
    pub async fn logout(&self, user_id: Uuid) -> Result<(), AppError> {
        let mut redis = self.redis.clone();
        let user_tokens_key = keys::user_tokens(&user_id);

        // Get all token IDs for this user
        let token_ids: Vec<String> = redis.smembers(&user_tokens_key).await?;

        // Delete all tokens
        for token_id in token_ids {
            let access_key = keys::access_token(&token_id);
            let refresh_key = keys::refresh_token(&token_id);
            let _: () = redis.del(&[&access_key, &refresh_key]).await?;
        }

        // Delete user tokens set
        let _: () = redis.del(&user_tokens_key).await?;

        Ok(())
    }

    /// Validate an access token and return claims.
    pub async fn validate_access_token(&self, token: &str) -> Result<Claims, AppError> {
        let claims = self.validate_token(token)?;

        if claims.token_type != "access" {
            return Err(AppError::JwtError("Invalid token type".to_string()));
        }

        // Check if token is in Redis (not revoked)
        let key = keys::access_token(&claims.jti);
        let mut redis = self.redis.clone();
        let exists: bool = redis.exists(&key).await?;
        if !exists {
            return Err(AppError::JwtError("Token has been revoked".to_string()));
        }

        Ok(claims)
    }

    /// Get user permissions by role ID.
    pub async fn get_user_permissions(&self, role_id: Uuid) -> Result<Vec<String>, AppError> {
        self.role_repo.get_permissions(role_id).await
    }

    /// Check if user has a specific permission.
    pub async fn has_permission(&self, role_id: Uuid, permission: &str) -> Result<bool, AppError> {
        let permissions = self.get_user_permissions(role_id).await?;
        Ok(permissions.iter().any(|p| p == permission))
    }

    // Private helper methods

    fn create_access_token(&self, user: &UserWithRole) -> Result<(String, String), AppError> {
        let jti = Uuid::new_v4().to_string();
        let now = Utc::now();
        let exp = now + Duration::hours(self.config.jwt_access_expiry_hours);

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role_id: user.role_id.to_string(),
            role_slug: user.role_slug.clone(),
            jti: jti.clone(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: "access".to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )?;

        Ok((token, jti))
    }

    fn create_refresh_token(&self, user: &UserWithRole) -> Result<(String, String), AppError> {
        let jti = Uuid::new_v4().to_string();
        let now = Utc::now();
        let exp = now + Duration::days(self.config.jwt_refresh_expiry_days);

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role_id: user.role_id.to_string(),
            role_slug: user.role_slug.clone(),
            jti: jti.clone(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: "refresh".to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )?;

        Ok((token, jti))
    }

    fn validate_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    async fn store_token(
        &self,
        jti: &str,
        user_id: &Uuid,
        token_type: &str,
        expiry_secs: i64,
    ) -> Result<(), AppError> {
        let mut redis = self.redis.clone();

        let key = match token_type {
            "access" => keys::access_token(jti),
            "refresh" => keys::refresh_token(jti),
            _ => return Err(AppError::InternalError("Invalid token type".to_string())),
        };

        // Store token with expiration
        let _: () = redis
            .set_ex(&key, user_id.to_string(), expiry_secs as u64)
            .await?;

        // Add to user's token set
        let user_tokens_key = keys::user_tokens(user_id);
        let _: () = redis.sadd(&user_tokens_key, jti).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        // Create a minimal service for testing password functions
        let _config = Config::default();

        // We can't easily test without a real Redis connection,
        // but we can test the password hashing logic is sound
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(b"test_password", &salt)
            .unwrap()
            .to_string();

        let parsed_hash = PasswordHash::new(&hash).unwrap();
        assert!(argon2
            .verify_password(b"test_password", &parsed_hash)
            .is_ok());
        assert!(argon2
            .verify_password(b"wrong_password", &parsed_hash)
            .is_err());
    }

    #[test]
    fn test_claims_serialization() {
        let claims = Claims {
            sub: "test-user-id".to_string(),
            email: "test@example.com".to_string(),
            role_id: "role-id".to_string(),
            role_slug: "admin".to_string(),
            jti: "token-id".to_string(),
            exp: 1234567890,
            iat: 1234567800,
            token_type: "access".to_string(),
        };

        let json = serde_json::to_string(&claims).unwrap();
        assert!(json.contains("test@example.com"));
        assert!(json.contains("admin"));
        assert!(json.contains("role_slug"));
    }
}
