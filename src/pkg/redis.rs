//! Redis connection management for JWT token storage.

use redis::{aio::ConnectionManager, Client};

/// Create a new Redis connection manager.
///
/// # Arguments
/// * `redis_url` - Redis connection URL
///
/// # Returns
/// A connection manager that automatically reconnects on failure.
pub async fn create_connection(redis_url: &str) -> Result<ConnectionManager, redis::RedisError> {
    let client = Client::open(redis_url)?;
    ConnectionManager::new(client).await
}

/// Redis key prefixes for different token types.
pub mod keys {
    /// Prefix for access tokens
    pub const ACCESS_TOKEN_PREFIX: &str = "access_token:";
    /// Prefix for refresh tokens
    pub const REFRESH_TOKEN_PREFIX: &str = "refresh_token:";
    /// Prefix for user tokens (stores all token IDs for a user)
    pub const USER_TOKENS_PREFIX: &str = "user_tokens:";

    /// Generate access token key.
    pub fn access_token(token_id: &str) -> String {
        format!("{}{}", ACCESS_TOKEN_PREFIX, token_id)
    }

    /// Generate refresh token key.
    pub fn refresh_token(token_id: &str) -> String {
        format!("{}{}", REFRESH_TOKEN_PREFIX, token_id)
    }

    /// Generate user tokens set key.
    pub fn user_tokens(user_id: &uuid::Uuid) -> String {
        format!("{}{}", USER_TOKENS_PREFIX, user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::keys::*;
    use uuid::Uuid;

    #[test]
    fn test_access_token_key() {
        let key = access_token("abc123");
        assert_eq!(key, "access_token:abc123");
    }

    #[test]
    fn test_refresh_token_key() {
        let key = refresh_token("xyz789");
        assert_eq!(key, "refresh_token:xyz789");
    }

    #[test]
    fn test_user_tokens_key() {
        let user_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let key = user_tokens(&user_id);
        assert_eq!(key, "user_tokens:550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn test_create_connection_invalid_url_format() {
        // Test that an invalid URL format fails at client creation (sync, no network)
        let result = redis::Client::open("not-a-valid-url");
        assert!(result.is_err());
    }
}
