//! Configuration management for the application.
//!
//! Loads configuration from environment variables with sensible defaults.

use std::env;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    /// Server host address
    pub host: String,
    /// Server port
    pub port: u16,
    /// Database connection URL
    pub database_url: String,
    /// Redis connection URL
    pub redis_url: String,
    /// JWT secret key
    pub jwt_secret: String,
    /// JWT access token expiry in hours
    pub jwt_access_expiry_hours: i64,
    /// JWT refresh token expiry in days
    pub jwt_refresh_expiry_days: i64,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// # Panics
    /// Panics if required environment variables are not set.
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a valid number"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_access_expiry_hours: env::var("JWT_ACCESS_EXPIRY_HOURS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .expect("JWT_ACCESS_EXPIRY_HOURS must be a valid number"),
            jwt_refresh_expiry_days: env::var("JWT_REFRESH_EXPIRY_DAYS")
                .unwrap_or_else(|_| "7".to_string())
                .parse()
                .expect("JWT_REFRESH_EXPIRY_DAYS must be a valid number"),
        }
    }

    /// Get the server address as a string.
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            database_url: "postgres://localhost/test".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            jwt_secret: "test-secret".to_string(),
            jwt_access_expiry_hours: 1,
            jwt_refresh_expiry_days: 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert_eq!(config.jwt_access_expiry_hours, 1);
        assert_eq!(config.jwt_refresh_expiry_days, 7);
    }

    #[test]
    fn test_server_addr() {
        let config = Config::default();
        assert_eq!(config.server_addr(), "0.0.0.0:3000");
    }

    #[test]
    fn test_from_env() {
        // Set environment variables for test
        env::set_var("HOST", "127.0.0.1");
        env::set_var("PORT", "8080");
        env::set_var("DATABASE_URL", "postgres://test");
        env::set_var("REDIS_URL", "redis://test:6379");
        env::set_var("JWT_SECRET", "test-jwt-secret");
        env::set_var("JWT_ACCESS_EXPIRY_HOURS", "2");
        env::set_var("JWT_REFRESH_EXPIRY_DAYS", "14");

        let config = Config::from_env();

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert_eq!(config.database_url, "postgres://test");
        assert_eq!(config.redis_url, "redis://test:6379");
        assert_eq!(config.jwt_secret, "test-jwt-secret");
        assert_eq!(config.jwt_access_expiry_hours, 2);
        assert_eq!(config.jwt_refresh_expiry_days, 14);

        // Clean up
        env::remove_var("HOST");
        env::remove_var("PORT");
        env::remove_var("DATABASE_URL");
        env::remove_var("REDIS_URL");
        env::remove_var("JWT_SECRET");
        env::remove_var("JWT_ACCESS_EXPIRY_HOURS");
        env::remove_var("JWT_REFRESH_EXPIRY_DAYS");
    }
}
